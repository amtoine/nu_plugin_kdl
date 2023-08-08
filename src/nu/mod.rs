use crate::KDL;
use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, SyntaxShape, Value};

impl Plugin for KDL {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("kdl open")
            .usage("open a file as KDL document")
            .required("file", SyntaxShape::String, "filename")
            .plugin_examples(vec![PluginExample {
                example: "kdl open foo.kdl".into(),
                description: "open a KDL file".into(),
                result: None,
            }])
            .category(Category::Experimental)]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match name {
            "kdl open" => self.open(call, input),
            _ => Err(LabeledError {
                label: "Plugin call with wrong name signature".into(),
                msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
                span: Some(call.head),
            }),
        }
    }
}
