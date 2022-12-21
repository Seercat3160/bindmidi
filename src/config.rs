use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Midi2keyConfig {
    pub(crate) bindings: HashMap<u8, HashMap<String, Vec<String>>>,
    pub(crate) verbose: bool,
}
