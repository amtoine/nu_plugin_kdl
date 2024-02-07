use nu_plugin::LabeledError;
use nu_protocol::Value;

use kdl::{KdlDocument, KdlEntry, KdlIdentifier, KdlNode, KdlValue};

pub(crate) fn build_document(document: &Value) -> Result<KdlDocument, LabeledError> {
    let mut doc = KdlDocument::new();

    doc.set_span(document.span());

    let nodes = doc.nodes_mut();

    let Value::Record { val: record, .. } = document else {
        return Err(LabeledError {
            label: "invalid input".to_string(),
            msg: "value not supported for a document, expected record".to_string(),
            span: None,
        });
    };

    for (col, val) in record.columns().zip(record.values()) {
        let node = build_node(col, val)?;
        nodes.push(node);
    }

    Ok(doc)
}

fn build_node(name: &str, node: &Value) -> Result<KdlNode, LabeledError> {
    let mut identifier = KdlIdentifier::from(name);
    identifier.set_repr(name);
    let mut kdl_node = KdlNode::new(identifier);

    kdl_node.set_span(node.span());

    kdl_node.clear_entries();
    kdl_node.clear_children();
    let entries = kdl_node.entries_mut();
    match node {
        Value::Nothing { .. } => {}
        Value::String { .. } | Value::Int { .. } | Value::Float { .. } | Value::Bool { .. } => {
            entries.push(build_entry(node)?)
        }
        Value::List { vals, .. } => {
            for val in vals {
                entries.push(build_entry(val)?)
            }
        }
        // TODO: implement when node is a record, i.e. with children
        _ => {
            return Err(LabeledError {
                label: "invalid input".to_string(),
                msg: "value not supported for a node, expected list, record, string, int, float, bool or null"
                    .to_string(),
                span: None,
            })
        }
    }

    Ok(kdl_node)
}

fn build_entry(entry: &Value) -> Result<KdlEntry, LabeledError> {
    let span = entry.span();

    let mut entry = match entry {
        Value::Record { val: record, .. } => {
            if record.len() != 1 {
                return Err(LabeledError {
                    label: "invalid input".to_string(),
                    msg: "entry is a record but has more than one key".to_string(),
                    span: Some(span),
                });
            }

            let value = record.values().next().unwrap();
            let name = record.columns().next().unwrap();

            let val = match value {
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
                        span: Some(value.span()),
                    });
                }
            };

            KdlEntry::new_prop(name.clone(), val.clone())
        }
        Value::String { val, .. } => KdlEntry::new(KdlValue::String(val.to_string())),
        Value::Int { val, .. } => KdlEntry::new(KdlValue::Base10(*val)),
        Value::Float { val, .. } => KdlEntry::new(KdlValue::Base10Float(*val)),
        Value::Bool { val, .. } => KdlEntry::new(KdlValue::Bool(*val)),
        Value::Nothing { .. } => KdlEntry::new(KdlValue::Null),
        _ => {
            return Err(LabeledError {
                label: "invalid input".to_string(),
                msg: "value not supported for an entry, expected record, string, int, float, bool or null".to_string(),
                span: None,
            })
        }
    };

    entry.set_span(span);
    entry.set_leading(" ");

    Ok(entry)
}
