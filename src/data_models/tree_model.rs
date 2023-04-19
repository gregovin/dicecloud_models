use std::collections::HashMap;
use std::collections::hash_map::{Values, ValuesMut};

use crate::data_models::generic_model::{CharacterVar, CreatureInfo, Icon, Identifier, PropType};
use crate::data_models::flat_model::{FlatCharacter,FlatProp};
use serde::{Serialize,Deserialize};
/// an iterator over references to the children of a node
pub type Children<'a> = Values<'a,String,TreeProp>;
/// An iterator over mutable references to the children of a node
pub type ChildrenMut<'a>= ValuesMut<'a,String,TreeProp>;
/// Represents a property as it appears on the tree, by building children under parents
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
    child_map: HashMap<String,TreeProp>
}
impl TreeProp{
    /// Build a tree prop from a flat prop with no children
    #[allow(clippy::use_self)]
    #[must_use]
    pub fn from_flat_prop(p: FlatProp)->TreeProp{
        Self{
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
            child_map: HashMap::new()
        }
    }
    #[allow(clippy::use_self)]
    fn into_flat_prop(self)->(FlatProp,Vec<TreeProp>){
        (FlatProp{
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
        },self.child_map.into_values().collect())
    }
    /// Consumes this property and add this prop and all children to a specified vector 
    pub fn flatten(self,working: &mut Vec<FlatProp>){
        let flt = self.into_flat_prop();
        working.push(flt.0);
        for prp in flt.1{
            prp.flatten(working);
        }
    }
    /// add a child to this node in the tree
    #[allow(clippy::use_self)]
    pub fn add_child(&mut self, child: TreeProp){
        self.child_map.insert(child.id.clone(),child);
    }
    /// recursively insert a property based on its ancestors
    #[allow(clippy::use_self)]
    pub fn recurse_insert<I>(&mut self, rel_path: I, prop: TreeProp)
    where I: IntoIterator<Item=String>
    {
        let mut piter = rel_path.into_iter();
        let id = piter.next();
        if let Some(t)=id{
            self.child_map.entry(t).or_default()
                .recurse_insert(piter, prop);
        } else {
            self.add_child(prop);
        }
    }
    /// Returns an iterator over references to the children
    #[must_use]
    pub fn children(&self)->Children{
        self.child_map.values()
    }
    /// Returns an iterator over mutible references to the children
    pub fn children_mut(&mut self)->ChildrenMut{
        self.child_map.values_mut()
    }
    /// Takes the children out from the node, returning them as a vec and leaving nothing behind
    pub fn take_children(&mut self)->Vec<Self>{
        std::mem::take(&mut self.child_map).into_values().collect()
    }
}
impl PartialOrd for TreeProp{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order.partial_cmp(&other.order)
    }
}
/// An iterator over references to the roots of a node
pub type Roots<'a> = Values<'a,String,TreeProp>;
/// An iterator over mutable references to the roots of a node
pub type RootsMut<'a> = ValuesMut<'a, String, TreeProp>;
/// Represents a full character whose properties are in tree form
#[derive(Serialize, Deserialize,PartialEq,Default,Clone)]
#[serde(rename_all="camelCase")]
pub struct TreeCharacter{
    pub creatures: Vec<CreatureInfo>,
    pub(crate) creature_properties_tmap: HashMap<String,TreeProp>,
    pub creature_variables: Vec<HashMap<String, CharacterVar>>
}
impl TreeCharacter{
    /// Build the tree from a flat character
    /// 
    /// # Examples
    #[cfg_attr(doctest, doc = " ````no_test")]
    /// ```
    /// use crate::FlatCharacter;
    /// let flat_character: FlatCharacter= {...} //get the flat character
    /// 
    /// let tree_character= TreeCharacter::build_tree(flat_character);
    /// 
    /// ````
    #[must_use]
    pub fn build_tree(ch: FlatCharacter)->Self{
        let creatures = ch.creatures;
        let mut creature_properties=ch.creature_properties;
        let creature_variables=ch.creature_variables;
        creature_properties.sort_by(|a,b|a.partial_cmp(b).expect("unreachable"));
        let mut creature_properties_tmap:HashMap<String, TreeProp>=HashMap::new();
        for prop in creature_properties{
            let path: Vec<String> = prop.ancestors.iter().map(|anc|anc.id.clone()).collect();
            if let Some(id) =  path.get(1){
                creature_properties_tmap.entry(id.clone())
                    .and_modify(|p: &mut TreeProp| 
                        p.recurse_insert(path,
                            TreeProp::from_flat_prop(prop)));
            } else {
                creature_properties_tmap.insert(prop.id.clone(),TreeProp::from_flat_prop(prop));
            }
        }
        Self{creatures,creature_properties_tmap,creature_variables}
    }
    /// Returns an iterator over references to the roots of the tree
    #[must_use]
    pub fn roots(&self)->Roots{
        self.creature_properties_tmap.values()
    }
    /// Returns an iterator over mutable references to the roots of the tree
    pub fn roots_mut(&mut self)->RootsMut{
        self.creature_properties_tmap.values_mut()
    }
    /// Takes the roots out of the tree, returning them as a list
    /// 
    /// Leaves the interal set empty
    pub fn take_roots(&mut self)->Vec<TreeProp>{
        std::mem::take(&mut self.creature_properties_tmap).into_values().collect()
    }
    /// Add a root to the tree
    pub fn add_root(&mut self, prop: TreeProp){
        self.creature_properties_tmap.insert(prop.id.clone(), prop);
    }
}