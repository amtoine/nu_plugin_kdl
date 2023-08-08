use nu_plugin::LabeledError;
use nu_protocol::{Span, Value};

use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue, KdlIdentifier};
use miette::SourceSpan;

fn span(value: &Value) -> Result<SourceSpan, LabeledError> {
    match value.span() {
        Ok(Span { start, end, .. }) => Ok(SourceSpan::new(start.into(), (end - start).into())),
        Err(_) => {
            return Err(LabeledError {
                label: "internal error".to_string(),
                msg: "Nushell value does not have a span".to_string(),
                span: None,
            })
        }
    }
}

pub(crate) fn build_document(document: &Value) -> Result<KdlDocument, LabeledError> {
    let mut doc = KdlDocument::new();

    doc.set_span(span(document)?);

    let nodes = doc.nodes_mut();

    // TODO: implement the else branch
    let Value::Record { cols, .. } = document else { todo!() };

    for col in cols {
        // FIXME: do not unwrap here
        let node = build_node(col, &document.get_data_by_key(col).unwrap()).unwrap();
        nodes.push(node);
    }

    Ok(doc)
}

fn build_node(name: &str, node: &Value) -> Result<KdlNode, LabeledError> {
    let mut identifier = KdlIdentifier::from(name);
    identifier.set_repr(name);
    let mut kdl_node = KdlNode::new(identifier);

    kdl_node.set_span(span(node)?);

    kdl_node.clear_entries();
    kdl_node.clear_children();
    let entries = kdl_node.entries_mut();
    match node {
        Value::Nothing { .. } => {}
        Value::String { .. } | Value::Int { .. } | Value::Float { .. } | Value::Bool { .. } => {
            entries.push(build_entry(node).unwrap())
        }
        Value::List { vals, .. } => {
            for val in vals {
                entries.push(build_entry(val).unwrap())
            }
        }
        // TODO: implement when node is a record, i.e. with children
        // TODO: default arm
        _ => todo!(),
    }

    Ok(kdl_node)
}

fn build_entry(entry: &Value) -> Result<KdlEntry, LabeledError> {
    let span = span(entry)?;

    let mut entry = match entry {
        Value::Record { cols, vals, .. } => {
            if cols.len() != 1 {
                return Err(LabeledError {
                    label: "invalid input".to_string(),
                    msg: "entry should be either a record with one key".to_string(),
                    span: entry.span().ok(),
                });
            }

            let val = match &vals[0] {
                Value::String { val, .. } => KdlValue::String(val.to_string()),
                Value::Int { val, .. } => KdlValue::Base10(*val),
                Value::Float { val, .. } => KdlValue::Base10Float(*val),
                Value::Bool { val, .. } => KdlValue::Bool(*val),
                Value::Nothing { .. } => KdlValue::Null,
                _ => {
                    return Err(LabeledError {
                        label: "invalid input".to_string(),
                        msg: "value not supported, expected string, int, float, bool or null"
                            .to_string(),
                        span: vals[0].span().ok(),
                    });
                }
            };

            KdlEntry::new_prop(cols[0].clone(), val.clone())
        }
        Value::String { val, .. } => KdlEntry::new(KdlValue::String(val.to_string())),
        Value::Int { val, .. } => KdlEntry::new(KdlValue::Base10(*val)),
        Value::Float { val, .. } => KdlEntry::new(KdlValue::Base10Float(*val)),
        Value::Bool { val, .. } => KdlEntry::new(KdlValue::Bool(*val)),
        Value::Nothing { .. } => KdlEntry::new(KdlValue::Null),
        // TODO: default arm
        _ => todo!(),
    };

    entry.set_span(span);

    Ok(entry)
}
