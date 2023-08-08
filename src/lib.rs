mod nu;

use miette::SourceSpan;
use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{Span, Value};

use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};

pub struct KDL;

fn parse_document(document: &KdlDocument) -> Value {
    let cols: Vec<String> = document
        .nodes()
        .iter()
        .map(|node| node.name().to_string())
        .collect();
    let vals = document.nodes().iter().map(parse_node).collect();
    let span = Span::new(
        document.span().offset(),
        document.span().offset() + document.len(),
    );

    Value::record(cols, vals, span)
}

fn build_document(document: &Value) -> Result<KdlDocument, LabeledError> {
    let mut doc = KdlDocument::new();

    let span = match document.span() {
        Ok(Span { start, end, .. }) => SourceSpan::new(start.into(), end.into()),
        Err(_) => SourceSpan::new(0.into(), 0.into()),
    };
    doc.set_span(span);

    // TODO: use real data here
    doc.set_leading("");
    doc.set_trailing("");

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

fn parse_node(node: &KdlNode) -> Value {
    let entries: Vec<Value> = node.entries().iter().map(parse_entry).collect();

    let span = Span::new(node.span().offset(), node.span().offset() + node.len());

    if let Some(children) = node.children() {
        let children = parse_document(children);

        if entries.is_empty() {
            return children;
        }

        let entries = if entries.len() == 1 {
            entries[0].clone()
        } else {
            // FIXME: use a real span
            Value::list(entries, Span::unknown())
        };

        Value::Record {
            cols: vec!["entries".to_string(), "children".to_string()],
            vals: vec![entries, children],
            span,
        }
    } else {
        if entries.is_empty() {
            // FIXME: use a real span
            Value::nothing(Span::unknown())
        } else if entries.len() == 1 {
            entries[0].clone()
        } else {
            // FIXME: use a real span
            Value::list(entries, Span::unknown())
        }
    }
}

fn build_node(name: &str, node: &Value) -> Result<KdlNode, LabeledError> {
    let mut kdl_node = KdlNode::new(name);

    // TODO: use real data
    kdl_node.set_trailing("");
    kdl_node.set_leading("");
    kdl_node.set_ty("");

    let span = match node.span() {
        Ok(Span { start, end, .. }) => SourceSpan::new(start.into(), end.into()),
        Err(_) => SourceSpan::new(0.into(), 0.into()),
    };
    kdl_node.set_span(span);

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

fn parse_entry(entry: &KdlEntry) -> Value {
    let span = Span::new(entry.span().offset(), entry.span().offset() + entry.len());

    let value = match entry.value() {
        KdlValue::RawString(val) => Value::string(val.to_string(), span),
        KdlValue::String(val) => Value::string(val.to_string(), span),
        KdlValue::Base2(val) => Value::int(*val, span),
        KdlValue::Base8(val) => Value::int(*val, span),
        KdlValue::Base10(val) => Value::int(*val, span),
        KdlValue::Base16(val) => Value::int(*val, span),
        KdlValue::Base10Float(val) => Value::float(*val, span),
        KdlValue::Bool(val) => Value::bool(*val, span),
        KdlValue::Null => Value::nothing(span),
    };

    match entry.name() {
        Some(name) => Value::Record {
            cols: vec![name.value().to_string()],
            vals: vec![value],
            span,
        },
        None => value,
    }
}

fn build_entry(entry: &Value) -> Result<KdlEntry, LabeledError> {
    let span = match entry.span() {
        Ok(Span { start, end, .. }) => SourceSpan::new(start.into(), end.into()),
        Err(_) => SourceSpan::new(0.into(), 0.into()),
    };

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

    // TODO: use true KdlEntry values here
    entry.set_ty("");
    entry.set_leading("");
    entry.set_trailing("");

    Ok(entry)
}

impl KDL {
    pub fn from(&self, _call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let doc: KdlDocument = input
            .as_string()
            .expect("input is not a string")
            .parse()
            .expect("failed to parse KDL");
        Ok(parse_document(&doc))
    }

    pub fn to(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let document = build_document(input)?;
        Ok(Value::string(document.to_string(), call.head))
    }
}
