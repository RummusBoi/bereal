use lazy_static::lazy_static;

pub struct EnvironmentVars {
    pub use_mocked_database: bool,
    pub database_url: String,
}

fn generate_env_vars() -> EnvironmentVars {
    println!("Generating env vars!");
    let env_vars = read_env_vars();
    return EnvironmentVars {
        use_mocked_database: get_env_value("use_mocked_database", &env_vars).unwrap() == "true",
        database_url: get_env_value("database_url", &env_vars).unwrap(),
    };
}

lazy_static! {
    pub static ref ENV_VARS: EnvironmentVars = generate_env_vars();
}

fn read_env_vars() -> Vec<(String, String)> {
    dotenv::from_path("./src/configuration/.env")
        .expect("Expected to find environment file at location /backend/src/configuration/.env");

    dotenv::vars().collect()
}

fn get_env_value(key: &'static str, env_vars: &Vec<(String, String)>) -> Option<String> {
    env_vars
        .iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| Some(v.clone()))
}

pub trait VectorTools<T>
where
    T: Clone,
{
    fn map<F, U>(&self, closure: F) -> Vec<U>
    where
        F: FnMut(&T) -> U;

    fn find<F>(&self, closure: F) -> Option<&T>
    where
        F: Fn(&T) -> bool;
    fn flat_map<F, U>(&self, closure: F) -> Vec<U>
    where
        F: FnMut(&T) -> Vec<U>;
}

impl<T> VectorTools<T> for Vec<T>
where
    T: Clone,
{
    fn map<F, U>(&self, closure: F) -> Vec<U>
    where
        F: FnMut(&T) -> U,
    {
        self.iter().map(closure).collect()
    }

    fn find<F>(&self, closure: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        self.iter().find(|elem| closure(elem))
    }

    fn flat_map<F, U>(&self, closure: F) -> Vec<U>
    where
        F: FnMut(&T) -> Vec<U>,
    {
        self.iter().flat_map(closure).collect()
    }
}
