mod nu;

use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{Span, Value};

use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};

pub struct KDL;

fn build_document(document: &KdlDocument) -> Result<Value, LabeledError> {
    eprintln!(
        "{} -> {}",
        document.span().offset(),
        document.span().offset() + document.len()
    );

    for node in document.nodes() {
        print_node(node);
    }

    Ok(Value::Nothing {
        span: Span::new(
            document.span().offset(),
            document.span().offset() + document.len(),
        ),
    })
}

fn print_node(node: &KdlNode) {
    eprintln!(
        "{}: {} -> {} ({}, {})",
        node.name(),
        node.span().offset(),
        node.span().offset() + node.len(),
        node.entries().len(),
        match node.children() {
            Some(_) => true,
            None => false,
        },
    );

    for entry in node.entries() {
        build_entry(entry);
    }

    if let Some(children) = node.children() {
        let _ = build_document(children);
    }
}

fn build_entry(entry: &KdlEntry) -> Value {
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

impl KDL {
    pub fn from(&self, _call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let doc: KdlDocument = input
            .as_string()
            .expect("input is not a string")
            .parse()
            .expect("failed to parse KDL");
        build_document(&doc)
    }
}
