//! Defines structures for DSL.
//! Think of it as an AST, used solely for representing the language structure.
//! The actual state representation is built in the compiler.

use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{
    formulas::{parse_formula, FormulaError},
    nodes::{FilterNode, LabelNode, Node, NodeProperties, UProbe},
    ProgGraph,
};

pub type NodeId = usize;

pub fn construct_prog(nodes: Vec<Elem>) -> Result<ProgGraph, FormulaError> {
    let mut prog = ProgGraph::new();

    let mut node_ids: HashMap<NodeId, NodeIndex<u32>> = HashMap::new();

    for node_desc in &nodes {
        let node = node_desc.construct_node()?;

        let node_id = node_desc.node_id().unwrap();
        let node_idx = prog.add_node(node);

        // assoc dsl node_id with the graph node index
        node_ids.insert(node_id, node_idx);

        // connect nodes in the graph
        for input in node_desc.inputs().unwrap().iter() {
            prog.add_edge(
                *node_ids.get(input).expect("node ID not found"),
                node_idx,
                (),
            );
        }
    }

    Ok(prog)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum Elem {
    Node {
        #[serde(rename = "@id")]
        id: NodeId,
        #[serde(rename = "@inputs")]
        inputs: Vec<NodeId>,
        name: Option<String>,
        node_type: String,
        #[serde(rename = "node_properties")]
        properties: HashMap<String, String>,
    },
}

impl Elem {
    pub fn node_id(&self) -> Option<NodeId> {
        match self {
            Elem::Node { id, .. } => Some(*id),
        }
    }

    pub fn inputs(&self) -> Option<&[NodeId]> {
        match self {
            Elem::Node { inputs, .. } => Some(inputs),
        }
    }

    fn construct_properties(
        props: &HashMap<String, String>,
    ) -> Result<NodeProperties, FormulaError> {
        let mut node_props = NodeProperties::new();

        for (key, val) in props.iter() {
            node_props.insert(key, parse_formula(val.as_str())?);
        }

        Ok(node_props)
    }

    pub fn construct_node(&self) -> Result<Node, FormulaError> {
        let Elem::Node {
            node_type,
            properties,
            id,
            ..
        } = self;

        let properties = Self::construct_properties(properties)?;

        let node = match node_type.as_str() {
            "UProbe" => Node::UProbe(UProbe::new(*id, properties)),
            "Filter" => Node::Filter(FilterNode::new(*id, properties)),
            "Label" => Node::Label(LabelNode::new(*id, properties)),
            _ => {
                return Err(FormulaError::Other(format!(
                    "unknown node type: {}",
                    node_type
                )));
            }
        };

        Ok(node)
    }
}
