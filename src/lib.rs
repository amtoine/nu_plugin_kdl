mod nu;

use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{Span, Value};

use kdl::{KdlDocument, KdlEntry, KdlNode};

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
        print_entry(entry);
    }

    if let Some(children) = node.children() {
        let _ = build_document(children);
    }
}

fn print_entry(entry: &KdlEntry) {
    eprintln!(
        "{:?}: {} ({} -> {})",
        entry.name(),
        entry.value(),
        entry.span().offset(),
        entry.span().offset() + entry.len(),
    );
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
