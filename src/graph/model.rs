use serde::{Deserialize, Serialize};

fn default_weight() -> f32 {
    1.0
}

/// Stable join from a graph node/edge to corpus chunks (titles survive re-ingest; hash chunk_ids do not).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRef {
    #[serde(default)]
    pub doc_id: Option<String>,
    pub doc_title: String,
    #[serde(default)]
    pub section_path: Option<Vec<String>>,
}

/// Named entity in the directed knowledge graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub source_refs: Vec<SourceRef>,
}

/// Directed edge. `source` / `target` are entity ids.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationship {
    pub id: String,
    pub source: String,
    pub target: String,
    pub rel_type: String,
    #[serde(default = "default_weight")]
    pub weight: f32,
    #[serde(default)]
    pub source_refs: Vec<SourceRef>,
}

/// On-disk JSON document (`data/graph_store.json`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphDocument {
    #[serde(default)]
    pub entities: Vec<Entity>,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTRACT_JSON: &str = r#"
{
  "entities": [
    {
      "id": "ent:principal_contradiction",
      "name": "主要矛盾",
      "aliases": ["principal contradiction", "矛盾的主要方面"],
      "domain": "mao",
      "source_refs": [
        {"doc_title": "矛盾论", "section_path": ["二"]}
      ]
    }
  ],
  "relationships": [
    {
      "id": "rel:contradiction-amdahl",
      "source": "ent:principal_contradiction",
      "target": "ent:amdahls_serial_fraction",
      "rel_type": "aligned_with",
      "source_refs": [
        {"doc_title": "矛盾论"},
        {"doc_title": "阿姆达尔定律 (Amdahl's Law)"}
      ]
    }
  ]
}
"#;

    #[test]
    fn graph_document_matches_cycle11_json_contract() {
        let doc: GraphDocument = serde_json::from_str(CONTRACT_JSON).unwrap();
        assert_eq!(doc.entities.len(), 1);
        assert_eq!(doc.entities[0].name, "主要矛盾");
        assert_eq!(doc.entities[0].source_refs[0].doc_title, "矛盾论");
        assert_eq!(doc.relationships.len(), 1);
        assert_eq!(doc.relationships[0].rel_type, "aligned_with");
        assert_eq!(doc.relationships[0].weight, 1.0);
        assert!(doc.relationships[0].source_refs[0].doc_id.is_none());
    }
}
