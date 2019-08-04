use serde::{Deserialize, Serialize};

use std::env;
use std::path::PathBuf;
use std::string::String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub elm_compiler: String,
    pub node: String,
    pub defaults: PathBuf,
}

pub const DEFAULT_HARNESS: &str = r#"
const { Elm } = require('./elm.js');

let app = Elm.Main.init();

if (app.ports.write !== undefined) {
    app.ports.write.subscribe(console.log);
}
"#;

impl Default for Config {
    fn default() -> Self {
        Config {
            elm_compiler: "elm".into(),
            node: "node".into(),
            defaults: env::current_dir().unwrap(),
        }
    }
}
