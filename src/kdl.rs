use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::Value;
pub struct KDL;

impl KDL {
    pub fn open(&self, call: &EvaluatedCall, _input: &Value) -> Result<Value, LabeledError> {
        eprintln!("opening KDL document");

        Ok(Value::Nothing { span: call.head })
    }
}
