use crate::KDL;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Type, Value};

impl Plugin for KDL {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("from kdl")
                .usage("TODO")
                .input_output_type(Type::String, Type::Any)
                .plugin_examples(vec![PluginExample {
                    example: "open foo.kdl | from kdl".into(),
                    description: "TODO".into(),
                    result: None,
                }])
                .category(Category::Experimental),
            PluginSignature::build("to kdl")
                .usage("TODO")
                .input_output_type(Type::Any, Type::String)
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
