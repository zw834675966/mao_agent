use crate::error::{Result, VectorError};
use crate::graph::model::{Entity, GraphDocument, Relationship, SourceRef};
use crate::index::JiebaTokenizer;
use crate::vector::persist::atomic_replace;
use petgraph::Direction;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

/// One entity after seed + directed hop expansion.
#[derive(Debug, Clone, PartialEq)]
pub struct GraphExpandHit {
    pub entity_id: String,
    pub name: String,
    pub hop: u8,
    pub paths: Vec<String>,
    pub source_refs: Vec<SourceRef>,
}

#[derive(Serialize, Deserialize)]
struct GraphFile {
    graph: DiGraph<Entity, Relationship>,
}

/// In-memory directed knowledge graph. Tokenizer is rebuilt on load (not in the snapshot).
pub struct GraphStore {
    graph: DiGraph<Entity, Relationship>,
    id_to_node: HashMap<String, NodeIndex>,
    tokenizer: JiebaTokenizer,
}

impl GraphStore {
    pub fn from_document(doc: GraphDocument) -> Self {
        let mut graph = DiGraph::new();
        let mut id_to_node = HashMap::new();
        for entity in doc.entities {
            if id_to_node.contains_key(&entity.id) {
                continue;
            }
            let id = entity.id.clone();
            let idx = graph.add_node(entity);
            id_to_node.insert(id, idx);
        }
        for rel in doc.relationships {
            let Some(&src) = id_to_node.get(&rel.source) else {
                continue;
            };
            let Some(&tgt) = id_to_node.get(&rel.target) else {
                continue;
            };
            graph.add_edge(src, tgt, rel);
        }
        Self {
            graph,
            id_to_node,
            tokenizer: JiebaTokenizer::new(),
        }
    }

    pub fn from_json_str(json: &str) -> Result<Self> {
        let doc: GraphDocument = serde_json::from_str(json)?;
        Ok(Self::from_document(doc))
    }

    pub fn from_json_file(path: &Path) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        Self::from_json_str(&json)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let file = GraphFile {
            graph: self.graph.clone(),
        };
        let bytes =
            bincode::serialize(&file).map_err(|e| VectorError::Serialization(e.to_string()))?;
        atomic_replace(path, &bytes)
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        let file: GraphFile = bincode::deserialize(&bytes)
            .map_err(|e| VectorError::Deserialization(e.to_string()))?;
        Ok(Self::from_graph(file.graph))
    }

    fn from_graph(graph: DiGraph<Entity, Relationship>) -> Self {
        let mut id_to_node = HashMap::new();
        for idx in graph.node_indices() {
            if let Some(entity) = graph.node_weight(idx) {
                id_to_node.insert(entity.id.clone(), idx);
            }
        }
        Self {
            graph,
            id_to_node,
            tokenizer: JiebaTokenizer::new(),
        }
    }

    pub fn node_for_entity(&self, id: &str) -> Option<NodeIndex> {
        self.id_to_node.get(id).copied()
    }

    pub fn entity_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn find_seed_entities(&self, query: &str) -> Vec<&Entity> {
        let tokens = self.tokenizer.cut_search(query);
        self.graph
            .node_weights()
            .filter(|e| entity_matches_query(e, query, &tokens))
            .collect()
    }

    /// Directed expansion from query seeds. `max_hops` 1..=2; 0 still returns seeds.
    pub fn expand(&self, query: &str, max_hops: u8) -> Vec<GraphExpandHit> {
        let hops = max_hops.min(2);
        let tokens = self.tokenizer.cut_search(query);
        let mut seeds: Vec<NodeIndex> = self
            .graph
            .node_indices()
            .filter(|&idx| {
                self.graph
                    .node_weight(idx)
                    .is_some_and(|e| entity_matches_query(e, query, &tokens))
            })
            .collect();
        seeds.sort_by_key(|i| i.index());

        let mut best: HashMap<String, GraphExpandHit> = HashMap::new();
        let mut queued: HashSet<NodeIndex> = HashSet::new();
        let mut queue: VecDeque<(NodeIndex, u8, Option<String>)> = VecDeque::new();
        for idx in seeds {
            if queued.insert(idx) {
                queue.push_back((idx, 0, None));
            }
        }

        while let Some((idx, hop, incoming_path)) = queue.pop_front() {
            let Some(entity) = self.graph.node_weight(idx) else {
                continue;
            };
            let entry = best
                .entry(entity.id.clone())
                .or_insert_with(|| GraphExpandHit {
                    entity_id: entity.id.clone(),
                    name: entity.name.clone(),
                    hop,
                    paths: Vec::new(),
                    source_refs: entity.source_refs.clone(),
                });
            if hop < entry.hop {
                entry.hop = hop;
            }
            if let Some(path) = incoming_path
                && !entry.paths.contains(&path)
            {
                entry.paths.push(path);
            }

            if hop >= hops {
                continue;
            }
            for edge in self.graph.edges_directed(idx, Direction::Outgoing) {
                let tgt = edge.target();
                if !queued.insert(tgt) {
                    continue;
                }
                let rel = edge.weight();
                let Some(src_e) = self.graph.node_weight(idx) else {
                    continue;
                };
                let Some(tgt_e) = self.graph.node_weight(tgt) else {
                    continue;
                };
                let path = format!("{} -{}-> {}", src_e.name, rel.rel_type, tgt_e.name);
                queue.push_back((tgt, hop + 1, Some(path)));
            }
        }

        let mut hits: Vec<GraphExpandHit> = best.into_values().collect();
        hits.sort_by(|a, b| {
            a.hop
                .cmp(&b.hop)
                .then_with(|| a.entity_id.cmp(&b.entity_id))
        });
        hits
    }
}

fn entity_matches_query(entity: &Entity, query: &str, tokens: &[String]) -> bool {
    if query == entity.name {
        return true;
    }
    for alias in &entity.aliases {
        if alias_matches(alias, query) {
            return true;
        }
    }
    for token in tokens {
        if entity.name.contains(token.as_str()) {
            return true;
        }
        if entity
            .aliases
            .iter()
            .any(|a| a.contains(token.as_str()) || alias_matches(a, token))
        {
            return true;
        }
    }
    false
}

fn alias_matches(alias: &str, query: &str) -> bool {
    if alias == query {
        return true;
    }
    alias.is_ascii() && query.is_ascii() && alias.eq_ignore_ascii_case(query)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_alias_is_case_insensitive() {
        assert!(alias_matches(
            "principal contradiction",
            "Principal Contradiction"
        ));
        assert!(!alias_matches("主要矛盾", "主要矛盾x"));
    }
}
