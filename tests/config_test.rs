use mao_agent::config::ProjectConfig;
use std::path::PathBuf;

#[test]
fn parse_reads_cohere_api_key() {
    let cfg = ProjectConfig::parse(
        r#"
[cohere]
api_key = "test-key-123"
"#,
    )
    .unwrap();
    assert_eq!(cfg.cohere_api_key(), Some("test-key-123"));
}

#[test]
fn parse_trims_and_rejects_empty_key() {
    let cfg = ProjectConfig::parse("[cohere]\napi_key = \"   \"\n").unwrap();
    assert_eq!(cfg.cohere_api_key(), None);
}

#[test]
fn parse_missing_section_has_no_key() {
    let cfg = ProjectConfig::parse("").unwrap();
    assert_eq!(cfg.cohere_api_key(), None);
}

#[test]
fn load_from_path_reads_toml_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("config.toml");
    std::fs::write(&path, "[cohere]\napi_key = \"from-file\"\n").unwrap();
    let cfg = ProjectConfig::load_from_path(&path).unwrap();
    assert_eq!(cfg.cohere_api_key(), Some("from-file"));
}

#[test]
fn load_from_path_missing_file_is_error() {
    let err =
        ProjectConfig::load_from_path(&PathBuf::from("no-such-mao-agent-config.toml")).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("no-such-mao-agent-config.toml"), "{msg}");
}
