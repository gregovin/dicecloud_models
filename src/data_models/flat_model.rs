use std::collections::HashMap;

use crate::data_models::generic_model::{CharacterVar, CreatureInfo, Icon, Identifier, PropType};
use crate::data_models::tree_model::TreeCharacter;
use serde::{Serialize,Deserialize};

/// Represents all the details of a property as represented by dicecloud
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
/// represents an entire character from dicecloud
/// 
/// Does so in a flat way(ie as returned by the API)
/// Use `creature_variables` if you just want to process the variables for a creature
/// 
/// Or `creature_properties` if you need the properties
#[derive(Serialize, Deserialize,PartialEq,Default,Clone)]
#[serde(rename_all="camelCase")]
pub struct FlatCharacter{
    pub creatures: Vec<CreatureInfo>,
    pub creature_properties: Vec<FlatProp>,
    pub creature_variables: Vec<HashMap<String, CharacterVar>>
}
impl FlatCharacter{
    /// Convert a character in tree form back to its standard flat form.
    /// 
    /// Usefull if you want to make changes in tree form and then re-export
    #[allow(clippy::use_self)]
    #[must_use]
    pub fn from_tree_char(tree_char: TreeCharacter)->FlatCharacter{
        let creatures= tree_char.creatures;
        let creature_variables=tree_char.creature_variables;
        let mut creature_properties: Vec<FlatProp> = Vec::new();
        for prop in tree_char.creature_properties_tmap.into_values(){
            prop.flatten(&mut creature_properties);
        }
        Self { creatures, creature_properties, creature_variables}
    }
    /// Convert json return from dcv2 into a flat character
    /// 
    /// # Errors
    /// If the json is not valid or doesn't fit the expected format
    /// # Examples
    #[cfg_attr(doctest, doc = " ````no_test")]
    /// ```
    /// use crate::FlatCharacter;
    /// let character_string = {...} //get the character json as a String, somehow
    /// 
    /// let character = FlatCharacter::from_json(&character_string);
    /// // process character
    /// ````

    #[cfg(feature="serde_json")]
    #[allow(clippy::use_self)]
    pub fn from_json(json: &str)->serde_json::Result<FlatCharacter>{
        serde_json::from_str(json)
    }
}