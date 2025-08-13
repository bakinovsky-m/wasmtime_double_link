#[allow(warnings)]
mod bindings;

use bindings::Guest;
use crate::bindings::wasi::cli::environment;
struct Component;

impl Guest for Component {
    fn get_str() -> String {
        panic!("FUCK YOU");
        let mut environment = environment::get_environment();
        match environment.pop() {
            Some((key, _value)) => {
                ("get_str".to_owned() + &key).to_string()
            }
            None => {
                "get_str".to_string()
            }
        }
    }
}

bindings::export!(Component with_types_in bindings);
