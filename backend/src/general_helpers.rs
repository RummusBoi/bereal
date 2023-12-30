use std::env;

pub fn get_env_var(key: String) -> Option<String> {
    env::vars()
        .into_iter()
        .find(|var| var.0 == key)
        .and_then(|(var, val)| Some(val))
}
