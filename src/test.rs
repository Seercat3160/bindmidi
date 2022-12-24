use crate::{config::Midi2keyConfig, CONFIG_VERSION};

#[test]
fn default_config_is_valid() {
    let default_config: Midi2keyConfig =
        serde_yaml::from_str(include_str!("../config.default.yml"))
            .expect("Default config file is invalid!");
    assert_eq!(
        default_config.version, CONFIG_VERSION,
        "Default config file has an incompatible version!"
    );
}
