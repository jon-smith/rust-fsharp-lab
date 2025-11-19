use std::env;

use thiserror::Error;

#[derive(Error, Debug)]
pub struct EnvVarError {
    pub var_name: String,
    pub source: env::VarError,
}

impl std::fmt::Display for EnvVarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to read environment variable '{}': {}",
            self.var_name, self.source
        )
    }
}

fn map_env_error(
    result: Result<String, env::VarError>,
    var_name: &str,
) -> Result<String, EnvVarError> {
    result.map_err(|e| EnvVarError {
        var_name: var_name.to_string(),
        source: e,
    })
}

pub fn get_env_var(var_name: &str) -> Result<String, EnvVarError> {
    map_env_error(env::var(var_name), var_name)
}
