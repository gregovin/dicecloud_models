use std::collections::HashMap;

use crate::data_models::generic_model::*;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize,PartialEq,Debug,Default,Clone)]
#[serde(rename_all="camelCase")]
pub struct TreeProp{
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
    pub migration_error: Vec<String>,
    #[serde(default)]
    pub child_map: HashMap<String,TreeProp>
}
impl TreeProp{
    pub fn from_flat_prop(p: FlatProp)->TreeProp{
        TreeProp{
            id: p.id,
            prop_type:p.prop_type,
            tags: p.tags,
            order:p.order,
            parent:p.parent,
            ancestors: p.ancestors,
            library_node_id: p.library_node_id,
            color: p.color,
            icon: p.icon,
            library_tags: p.library_tags,
            deactivated_by_toggle: p.deactivated_by_toggle,
            deactivated_by_ancestor: p.deactivated_by_ancestor,
            inactive: p.inactive,
            removed: p.removed,
            removed_at: p.removed_at,
            removed_with: p.removed_with,
            dirty: p.dirty,
            migration_error: p.migration_error,
            childern: HashMap::new()
        }
    }
    pub fn as_flat_prop(self)->FlatProp{
        FlatProp{
            id: self.id,
            prop_type:self.prop_type,
            tags: self.tags,
            order: self.order,
            parent: self.parent,
            ancestors: self.ancestors,
            library_node_id: self.library_node_id,
            color: self.color,
            icon: self.icon,
            library_tags: self.library_tags,
            deactivated_by_toggle: self.deactivated_by_toggle,
            deactivated_by_ancestor: self.deactivated_by_ancestor,
            inactive: self.inactive,
            removed: self.removed,
            removed_at: self.removed_at,
            removed_with: self.removed_with,
            dirty: self.dirty,
            migration_error: self.migration_error
        }
    }
    pub fn add_child(&mut self, child: TreeProp){
        self.child_map.insert(child.id.clone(),child);
    }
    pub fn recurse_insert(&mut self, rel_path: I, prop: TreeProp)
    where I: IntoIterator<String>
    {
        let mut piter = rel_path.into_iter();
        let id = piter.next();
        if let Some(t)=id{
            self.child_map.entry(t).and_modify(|p| p.recurse_insert(piter,prop))
                .or_insert(|| {
                    let p = TreeProp::default();
                    p.recurse_insert(piter,prop);
                    p})
        } else {
            self.add_child(prop);
        }
    }
}
impl PartialOrd for TreeProp{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}
#[derive(Serialize, Deserialize,PartialEq,Default,Clone)]
#[serde(rename_all="camelCase")]
pub struct TreeCharacter{
    pub creatures: Vec<CreatureInfo>,
    pub creature_properties_tmap: HashMap<String,TreeProp>,
    pub creature_variables: Vec<HashMap<String, CharacterVar>>
}
impl TreeCharacter{
    pub fn build_tree(ch: FlatCharacter)->TreeCharacter{
        let creatures = ch.creatures;
        let mut creature_properties=ch.creature_properties;
        let creature_variables=ch.creature_variables;
        creature_properties.sort();
        let mut creature_properties_tmap=HashMap::new();
        for prop in creature_properties{
            let mut path = prop.ancestors.iter().cloned().map(|anc|anc.id);
            let _ = path.next(); //always the character id
            if let Some(id) =  path.next(){
                creature_properties_tmap.entry(id)
                    .and_modify(|p| p.recurse_insert(path,TreeProp::from_flat_prop(prop)));
            } else {
                creature_properties_tmap.insert(prop.id.clone(),TreeProp::from_flat_prop(prop));
            }
        }
        TreeCharacter{creatures,creature_properties_tmap,creature_variables}
    }
}