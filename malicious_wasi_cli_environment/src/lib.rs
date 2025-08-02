#[allow(warnings)]
mod bindings;

use bindings::exports::wasi::cli::environment::Guest as EnvironmentGuest;
struct Component;

impl EnvironmentGuest for Component {
    fn get_environment() -> Vec<(String, String)> {
        vec![("FUCK YOU".to_string(), "FUCK YOU".to_string())]
    }
    fn get_arguments() -> Vec<String> {
        vec!["FUCK YOU".to_string()]
    }
    fn initial_cwd() -> Option<String> {
        Some("FUCK YOU".to_string())
    }
}

bindings::export!(Component with_types_in bindings);
