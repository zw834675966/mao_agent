use mao_agent::graph::{GraphDocument, GraphStore};

const FIXTURE: &str = r#"
{
  "entities": [
    {
      "id": "ent:principal_contradiction",
      "name": "主要矛盾",
      "aliases": ["principal contradiction", "矛盾的主要方面"],
      "domain": "mao",
      "source_refs": [{"doc_title": "矛盾论", "section_path": ["二"]}]
    },
    {
      "id": "ent:amdahls_serial_fraction",
      "name": "阿姆达尔定律 (Amdahl's Law)",
      "aliases": ["Amdahl's Law"],
      "domain": "hacker_laws",
      "source_refs": [{"doc_title": "阿姆达尔定律 (Amdahl's Law)"}]
    }
  ],
  "relationships": [
    {
      "id": "rel:contradiction-amdahl",
      "source": "ent:principal_contradiction",
      "target": "ent:amdahls_serial_fraction",
      "rel_type": "aligned_with",
      "weight": 1.0,
      "source_refs": [
        {"doc_title": "矛盾论"},
        {"doc_title": "阿姆达尔定律 (Amdahl's Law)"}
      ]
    },
    {
      "id": "rel:orphan",
      "source": "ent:missing",
      "target": "ent:amdahls_serial_fraction",
      "rel_type": "aligned_with",
      "weight": 1.0
    }
  ]
}
"#;

#[test]
fn from_json_skips_unknown_edge_endpoints() {
    let store = GraphStore::from_json_str(FIXTURE).unwrap();
    assert_eq!(store.entity_count(), 2);
    assert_eq!(store.edge_count(), 1);
}

#[test]
fn seed_主要矛盾_and_hop1_aligned_with() {
    let store = GraphStore::from_json_str(FIXTURE).unwrap();
    let seeds = store.find_seed_entities("主要矛盾");
    assert_eq!(seeds.len(), 1);
    assert_eq!(seeds[0].id, "ent:principal_contradiction");

    let hits = store.expand("主要矛盾", 1);
    assert!(
        hits.iter()
            .any(|h| h.entity_id == "ent:principal_contradiction" && h.hop == 0),
        "{hits:?}"
    );
    let amdahl = hits
        .iter()
        .find(|h| h.entity_id == "ent:amdahls_serial_fraction")
        .expect("hop-1 neighbor");
    assert_eq!(amdahl.hop, 1);
    assert!(
        amdahl
            .paths
            .iter()
            .any(|p| p.contains("aligned_with") && p.contains("阿姆达尔")),
        "{:?}",
        amdahl.paths
    );
}

#[test]
fn ascii_alias_seed() {
    let store = GraphStore::from_json_str(FIXTURE).unwrap();
    let seeds = store.find_seed_entities("Principal Contradiction");
    assert_eq!(seeds.len(), 1);
}

#[test]
fn bincode_round_trip() {
    let store = GraphStore::from_json_str(FIXTURE).unwrap();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("graph_store.bin");
    store.save_to_file(&path).unwrap();
    let loaded = GraphStore::load_from_file(&path).unwrap();
    assert_eq!(loaded.entity_count(), store.entity_count());
    assert_eq!(loaded.edge_count(), store.edge_count());
    assert_eq!(loaded.find_seed_entities("主要矛盾").len(), 1);
}

#[test]
fn loads_committed_golden_graph() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("evals/graph/golden_graph.json");
    let store = GraphStore::from_json_file(&path).unwrap();
    assert!(store.entity_count() >= 2);
    assert!(!store.find_seed_entities("主要矛盾").is_empty());
    let hits = store.expand("主要矛盾", 1);
    assert!(
        hits.iter()
            .any(|h| h.name.contains("阿姆达尔") && h.hop == 1),
        "{hits:?}"
    );
}

#[test]
fn from_document_empty_is_ok() {
    let store = GraphStore::from_document(GraphDocument::default());
    assert_eq!(store.entity_count(), 0);
    assert!(store.expand("主要矛盾", 2).is_empty());
}
