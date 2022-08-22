pub struct ModDetails {
    pub default_name: Option<String>,
    pub url: String,
    pub mod_path: String
}

impl ModDetails {
    pub fn new(default_name: Option<String>, url: String, mod_path: String) -> ModDetails {
        ModDetails { default_name, url, mod_path }
    }
}
