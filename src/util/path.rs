use std::path;

pub fn cargo_path() -> Option<path::PathBuf> {
    option_env!("CARGO_MANIFEST_DIR").map(|env_path| {
        let mut res_path = path::PathBuf::from(env_path);
        res_path.push("resources");
        res_path
    })
}
