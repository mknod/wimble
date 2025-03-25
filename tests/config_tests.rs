use wimble::config::load_config;

#[test]
fn test_load_config_ok() {
    // You'll need a `config.toml` or `config` file present in test root.
    let config = load_config();
    assert!(config.is_ok());

    let config = config.unwrap();
    assert!(config.global.placeholder);
}