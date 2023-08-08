mod from;
mod to;

use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Type, Value};

use kdl::KdlDocument;

pub struct KDL;

impl KDL {
    pub fn from(&self, _call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let doc = match input
            .as_string()
            .expect("input is not a string")
            .parse::<KdlDocument>()
        {
            Ok(document) => document,
            Err(err) => {
                return Err(LabeledError {
                    label: err.label.unwrap_or("invalid format").to_string(),
                    msg: err.help.unwrap_or("input to `kdl from` has invalid KDL format").to_string(),
                    span: None,
                })
            }
        };

        Ok(from::parse_document(&doc))
    }

    pub fn to(&self, call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
        let document = to::build_document(input)?;
        Ok(Value::string(document.to_string(), call.head))
    }
}

impl Plugin for KDL {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("from kdl")
                .usage("TODO")
                .input_output_type(Type::String, Type::Record(vec![]))
                .plugin_examples(vec![PluginExample {
                    example: "open foo.kdl | from kdl".into(),
                    description: "TODO".into(),
                    result: None,
                }])
                .category(Category::Experimental),
            PluginSignature::build("to kdl")
                .usage("TODO")
                .input_output_type(Type::Record(vec![]), Type::String)
                .plugin_examples(vec![PluginExample {
                    example: "{foo: [1, 2, 3]} | to kdl".into(),
                    description: "TODO".into(),
                    result: None,
                }])
                .category(Category::Experimental),
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "from kdl" => self.from(call, input),
            "to kdl" => self.to(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}
