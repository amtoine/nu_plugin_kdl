use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_kdl::KDL;

fn main() {
    serve_plugin(&mut KDL{}, MsgPackSerializer {})
}
