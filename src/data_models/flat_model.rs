use std::collections::HashMap;

use crate::data_models::generic_model::*;
use crate::data_models::tree_model::TreeCharacter;
use serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize,PartialEq,Debug,Default,Clone)]
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
    pub inactive: bool,
    #[serde(default)]
    pub removed: bool,
    #[serde(default)]
    pub removed_at: Option<String>,
    #[serde(default)]
    pub removed_with: Option<String>,
    #[serde(default)]
    pub dirty: Option<bool>,
    #[serde(default,rename="_migrationError")]
    pub migration_error: Vec<String>
}
impl PartialOrd for FlatProp{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}
#[derive(Serialize, Deserialize,PartialEq,Default,Clone)]
#[serde(rename_all="camelCase")]
pub struct FlatCharacter{
    pub creatures: Vec<CreatureInfo>,
    pub creature_properties: Vec<FlatProp>,
    pub creature_variables: Vec<HashMap<String, CharacterVar>>
}
impl FlatCharacter{
    pub fn from_tree_char(tree_char: TreeCharacter)->FlatCharacter{
        let creatures= tree_char.creatures;
        let creature_variables=tree_char.creature_variables;
        let mut creature_properties: Vec<FlatProp> = Vec::new();
        for prop in tree_char.creature_properties_tmap.into_values(){
            prop.flatten(&mut creature_properties);
        }
        FlatCharacter { creatures, creature_properties, creature_variables}
    }
}