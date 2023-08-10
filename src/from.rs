use nu_protocol::{Span, Value};

use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};

pub(crate) fn parse_document(document: &KdlDocument) -> Value {
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

fn parse_node(node: &KdlNode) -> Value {
    let mut entries: Vec<Value> = node.entries().iter().map(parse_entry).collect();

    let span = Span::new(node.span().offset(), node.span().offset() + node.len());

    if let Some(children) = node.children() {
        entries.push(parse_document(children))
    }

    if entries.is_empty() {
        Value::nothing(span)
    } else if entries.len() == 1 {
        entries[0].clone()
    } else {
        Value::list(entries, span)
    }
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
