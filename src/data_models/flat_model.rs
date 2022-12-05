use std::collections::HashMap;

use crate::data_models::generic_model::*;
use serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize,PartialEq,Debug,Default)]
#[serde(rename_all="camelCase")]
pub struct FlatProp{
    #[serde(rename="_id")]
    pub id: String,
    #[serde(flatten)]
    pub prop_type: PropType,
    pub tags: Vec<String>,
    pub order: usize,
    pub parent: Identifier,
    pub ancestors: Vec<Identifier>,
    #[serde(default)]
    pub library_node_id: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon: Option<Icon>,
    #[serde(default)]
    pub library_tags: Vec<String>,
    #[serde(default)]
    pub deactivated_by_toggle: Option<bool>,
    #[serde(default)]
    pub deactivated_by_ancestor: Option<bool>,
    #[serde(default)]
    pub inactive: Option<bool>,
    #[serde(default)]
    pub removed: Option<bool>,
    #[serde(default)]
    pub removed_at: Option<String>,
    #[serde(default)]
    removed_with: Option<String>
}
impl PartialOrd for FlatProp{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}
#[derive(Serialize, Deserialize,PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Character{
    creatures: Vec<CreatureInfo>,
    creature_properties: Vec<FlatProp>,
    creature_variables: HashMap<String, serde_json::Value>
}
impl Default for Character{
    fn default() -> Self {
        Character { creatures: vec![], creature_properties: vec![], creature_variables: HashMap::new() }
    }
}