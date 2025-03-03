use std::io::Error;
use std::path::PathBuf;

fn base_dir(env_key: &str, default_base: &str) -> Result<PathBuf, Error> {
    let base_home: PathBuf = match std::env::var(env_key) {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            let home = std::env::home_dir().ok_or(Error::new(
                std::io::ErrorKind::NotFound,
                "Home directory not found",
            ))?;
            home.join(default_base)
        }
    };
    Ok(base_home)
}

pub fn config_home() -> Result<PathBuf, Error> {
    base_dir("XDG_CONFIG_HOME", ".config")
}

pub fn cache_home() -> Result<PathBuf, Error> {
    base_dir("XDG_CACHE_HOME", ".cache")
}

pub fn data_home() -> Result<PathBuf, Error> {
    base_dir("XDG_DATA_HOME", ".local/share")
}

pub fn state_home() -> Result<PathBuf, Error> {
    base_dir("XDG_STATE_HOME", ".local/state")
}

pub fn runtime_home() -> Result<PathBuf, Error> {
    base_dir("XDG_RUNTIME_DIR", ".local/run")
}

pub fn binary_home() -> Result<PathBuf, Error> {
    base_dir("XDG_BIN_HOME", ".local/bin")
}

pub fn app_config_home(name: &str) -> Result<PathBuf, Error> {
    Ok(config_home()?.join(name))
}

pub fn app_cache_home(name: &str) -> Result<PathBuf, Error> {
    Ok(cache_home()?.join(name))
}
pub fn app_data_home(name: &str) -> Result<PathBuf, Error> {
    Ok(data_home()?.join(name))
}
pub fn app_state_home(name: &str) -> Result<PathBuf, Error> {
    Ok(state_home()?.join(name))
}
pub fn app_runtime_home(name: &str) -> Result<PathBuf, Error> {
    Ok(runtime_home()?.join(name))
}
