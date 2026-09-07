//! C3: Domain enum serialization, default, and category mapping.

use mao_agent::corpus::Domain;

#[test]
fn test_domain_serde_roundtrip() {
    // History
    let domain = Domain::History;
    let json = serde_json::to_string(&domain).unwrap();
    assert_eq!(json, "\"history\"");
    let back: Domain = serde_json::from_str(&json).unwrap();
    assert_eq!(back, domain);

    // Engineering
    let domain = Domain::Engineering;
    let json = serde_json::to_string(&domain).unwrap();
    assert_eq!(json, "\"engineering\"");

    // Any
    let domain = Domain::Any;
    let json = serde_json::to_string(&domain).unwrap();
    assert_eq!(json, "\"any\"");
}

#[test]
fn test_domain_any_is_default() {
    let domain: Domain = Default::default();
    assert_eq!(domain, Domain::Any);
}

#[test]
fn test_domain_from_category() {
    assert_eq!(Domain::from_category("history"), Domain::History);
    assert_eq!(Domain::from_category("HISTORY"), Domain::History);
    assert_eq!(Domain::from_category("engineering"), Domain::Engineering);
    assert_eq!(Domain::from_category("哲学"), Domain::Any);
    assert_eq!(Domain::from_category(""), Domain::Any);
    assert_eq!(Domain::from_category("unknown"), Domain::Any);
}
