use std::{env::VarError, str::FromStr};

pub fn load_from_env<K>(key: &str) -> Result<K, VarError>
where
    K: FromStr,
    K::Err: std::fmt::Debug,
{
    let variable = std::env::var(key)?;
    let parsed = variable
        .parse::<K>()
        .expect("Failed to parse environment variable");

    Ok(parsed)
}
