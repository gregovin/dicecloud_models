use std::default::Default;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize,PartialEq,Debug,Default,Hash)]
pub struct DeathSaveInfo{
    pub pass: usize,
    pub fail: usize,
    #[serde(rename = "canDeathSave")]
    pub can_death_save: bool,
    pub stable: bool
}

#[derive(Serialize,Deserialize, PartialEq,Debug,Default,Hash)]
pub struct DenormalizedStats{
    pub xp: usize,
    #[serde(rename = "milestoneLevels")]
    pub milestone_levels: usize,
}

#[derive(Serialize,Deserialize,Default, PartialEq,Debug,Hash)]
#[serde(rename_all="camelCase")]
pub struct Settings{
    #[serde(default)]
    pub show_tree_tab: bool,
    #[serde(default)]
    pub hide_rest_buttons: bool,
    #[serde(default)]
    pub hide_unused_stats: bool,
    #[serde(default)]
    pub hide_spells_tab: bool
}

#[derive(Serialize, Deserialize,PartialEq,Debug,Default,Hash)]
#[serde(rename_all="camelCase")]
pub struct CreatureInfo{
    #[serde(rename = "_id")]
    pub id: String,
    pub owner: String,
    pub name: String,
    pub gender: String,
    pub alignment: String,
    pub allowed_libraries: Vec<String>,
    pub allowed_library_collections: Vec<String>,
    pub death_save: DeathSaveInfo,
    pub denormalized_stats: DenormalizedStats,
    #[serde(rename = "type")]
    pub typ: String,
    //damageMultipliers and variables properties omitted
    #[serde(default)]
    pub settings: Settings,
    pub readers: Vec<String>,
    pub writers: Vec<String>,
    pub public: bool,
    #[serde(default)]
    pub picture: Option<String>,
    #[serde(default)]
    pub avatar_picture: Option<String>
}
#[derive(Serialize,Deserialize,PartialEq,Debug)]
#[serde(untagged,rename_all="camelCase")]
pub enum PropVal{
    Boolean(bool),
    Number(i64),
    Fraction(f64),
    Str(String),
}
impl Default for PropVal{
    fn default()->PropVal{
        PropVal::Number(0)
    }
}
#[derive(Serialize,Deserialize,PartialEq,Debug,Default)]
pub struct ValWrap{
    pub value: PropVal
}
#[derive(Serialize,Deserialize,PartialEq,Debug)]
#[serde(rename_all="camelCase",tag="parseType")]
pub enum ParseNode{
    Accessor{path: Vec<String>,name: String},
    Array{values: Vec<Box<ParseNode>>},
    Call{#[serde(rename="functionName")]function_name: String,args: Vec<Box<ParseNode>>},
    Constant{#[serde(rename="valueType")]value_type: String, value: PropVal},
    If{condition: Box<ParseNode>,consequent: Box<ParseNode>,alternative: Box<ParseNode>},
    Index{array: Box<ParseNode>,index:Box<ParseNode>},
    Operator{left: Box<ParseNode>,right: Box<ParseNode>,operator: String,#[serde(rename="fn")] fun: String},
    Not{right: Box<ParseNode>},
    Parenthesis{content: Box<ParseNode>},
    Roll{left: Box<ParseNode>, right: Box<ParseNode>},
    Symbol{name: String},
    UnaryOperator{operator: String, right: Box<ParseNode>},
}
impl Default for ParseNode{
    fn default()->ParseNode{
        ParseNode::Constant{value_type:"number".to_string(),value: PropVal::default()}
    }
}
#[derive(Serialize,Deserialize,PartialEq,Debug,Default,Hash)]
pub struct ParseError{
    #[serde(rename="type")]
    pub typ: String,
    pub message: String,
}
#[derive(Serialize,Deserialize,PartialEq,Debug, Default)]
#[serde(rename_all="camelCase")]
pub struct Calculation{
    pub calculation: String,
    #[serde(rename="_key")]
    pub key: String,
    #[serde(rename="type")]
    pub typ: String,
    pub hash: i64,
    pub parse_node: ParseNode,
    pub errors: Vec<ParseError>,
    pub value: PropVal,
}
#[derive(Serialize,Deserialize,PartialEq,Debug, Default)]
#[serde(rename_all="camelCase")]
pub struct ExtendedCalc{
    #[serde(flatten)]
    pub calculation: Calculation,
    #[serde(default)]
    pub base_value: Option<PropVal>,
    #[serde(default)]
    pub effects: Vec<Effect>
}
#[derive(Serialize,Deserialize,PartialEq,Debug, Default,Hash)]
#[serde(rename_all="camelCase")]
pub struct Identifier{
    pub id: String,
    pub collection: String,
}
#[derive(Serialize,Deserialize,PartialEq,Debug, Default)]
#[serde(rename_all="camelCase")]
pub struct CalculatedText{
    pub text: String,
    pub value: String,
    pub hash: i64,
    pub inline_calculations: Vec<Calculation>
}
#[derive(Serialize,Deserialize,PartialEq,Debug, Default)]
#[serde(rename_all="camelCase")]
pub struct Effect{
    #[serde(rename="_id")]
    pub id: String,
    pub name: String,
    pub operation: String,
    pub amount:ValWrap,
    #[serde(default,rename="type")]
    pub typ: Option<String>,
}
#[derive(Serialize,Deserialize,PartialEq,Debug,Default)]
#[serde(rename_all="camelCase")]
pub struct ConsumedItem{
    #[serde(rename="_id")]
    pub id: String,
    pub tag: String,
    pub quantity: Calculation,
}
#[derive(Serialize,Deserialize,PartialEq,Debug,Default)]
#[serde(rename_all="camelCase")]
pub struct ConsumedResource{
    #[serde(rename="_id")]
    pub id: String,
    pub variable_name: String,
    pub quantity: Calculation,
    pub available: i64,
    pub stat_name: String,
}
#[derive(Serialize,Deserialize,PartialEq,Debug,Default)]
#[serde(rename_all="camelCase")]
pub struct Resource{
    items_consumed: Vec<ConsumedItem>,
    attributes_consumed: Vec<ConsumedResource>,
}
#[derive(Serialize,Deserialize,PartialEq,Debug,Default)]
#[serde(rename_all="camelCase")]
pub struct Icon{
    pub name: String,
    pub shape: String,
}
#[derive(Serialize,Deserialize,PartialEq,Debug)]
#[serde(rename_all="camelCase",tag="branchType")]
pub enum BranchType{
    EachTarget{},
    FailedSave{},
    Hit{},
    If{condition: Calculation},
    Index{condition: Calculation},
    Miss{},
    SuccessfulSave{},
    Random{}
}
#[derive(Serialize,Deserialize,PartialEq,Debug)]
#[serde(rename_all="camelCase",tag="type")]
pub struct PointBuyRow{
    #[serde(rename="_id")]
    id: String,
    name: String,
    variable_name: String,
    value: i64,
    spent: i64
}
#[derive(Serialize,Deserialize,PartialEq,Debug)]
#[serde(rename_all="camelCase",tag="type")]
pub enum PropType{
    Action{#[serde(rename="actionType")] action_type: String,target: String,
        resources: Resource,name: String,#[serde(default)] summary: Option<CalculatedText>, 
        #[serde(default)] description: Option<CalculatedText>,
        #[serde(default,rename="variableName")] variable_name: Option<String>, 
        #[serde(default,rename="attackRoll")] attack_roll: Option<ExtendedCalc>},
    Adjustment{target: String,operation: String, stat: String, amount: Calculation},//attribute damage for some reason
    Attribute{name: String, #[serde(rename="variableName")] variable_name: String, 
        #[serde(rename="attributeType")] attribute_type: String,
        #[serde(rename="baseValue")] base_value: Calculation,total: PropVal, value: PropVal,
        #[serde(default)] damage: Option<PropVal>, effects: Vec<Effect>,
        #[serde(default)] modifier: Option<i64>},
    Buff{target: String, name: String, description: CalculatedText,
        #[serde(rename="skipCrystalization",default)]skip_crystalization: Option<bool>,
        #[serde(rename="hideRemoveButton",default)]hide_remove_button:Option<bool>,
        #[serde(default)]silent:Option<bool>},
    BuffRemover{target: String, #[serde(rename="removeAll",default)]remove_all:Option<bool>,name:String,
        #[serde(rename="targetParentBuff",default)]target_parent_buff:Option<bool>,
        #[serde(default)]silent:bool},
    Branch{#[serde(flatten)] branch_type: BranchType},
    Class{name: String, #[serde(rename="variableName")] variable_name: String,
        #[serde(rename="extraTags")] extra_tags: Vec<String>,
        #[serde(rename="slotCondition",default)] slot_condition: Option<Calculation>,
        level: i64},
    ClassLevel{name: String, level: i64, #[serde(rename="variableName")] variable_name: String},
    Constant{name: String, #[serde(rename="variableName")] variable_name: String, calculation: String,
        errors: Vec<ParseError>},
    Container{name: String, #[serde(default)] carried: bool,#[serde(default)] value: f64,
        #[serde(default)] weight: f64, #[serde(rename="carriedValue",default)] carried_value: f64,
        #[serde(rename="carriedWeight",default)] carried_weight: f64,
        #[serde(rename="contentsValue",default)] contents_value: f64,
        #[serde(rename="contentsWeight",default)] contents_weight: f64},
    Damage{target: String, #[serde(rename="damageType")] damage_type: String,
        amount: ExtendedCalc,#[serde(default)] silent: bool},
    DamageMultiplier{name: String, #[serde(rename="damageTypes")] damage_types: Vec<String>,value: f64,
        #[serde(rename="excludeTags",default)] exclude_tags: Vec<String>,
        #[serde(rename="includeTags",default)] include_tags: Vec<String>},
    Effect{name: String, operation: String,stats: Vec<String>,amount: Calculation,
        #[serde(rename="targetByTags",default)] target_by_tags: bool,
        #[serde(rename="targetTags",default)] target_tags: Option<Vec<String>>,
        #[serde(rename="extraTags",default)] extra_tags: Vec<String>,
        #[serde(rename="targetField",default)] target_field: Option<String>},
    Feature{name: String, #[serde(default)] sumary: Option<CalculatedText>,
        #[serde(default)] description: Option<CalculatedText>},
    Folder{name: String, #[serde(rename="groupStats",default)] group_stats: bool,
        #[serde(rename="hideStatsGroup",default)] hide_stats_group: Option<bool>,
        #[serde(default)] location: Option<String>, #[serde(default)] tab: Option<String>},
    Item{name: String, plural: String, quantity: i64, equipped: bool,#[serde(default)] value: f64,
        #[serde(default)] weight: f64, #[serde(rename="showIncrement",default)] show_increment: bool,
        #[serde(default)] description: Option<CalculatedText>,
        #[serde(rename="requiresAttunement",default)] requires_attunement: bool,
        #[serde(default)] attuned: Option<bool>},
    Note{name: String, #[serde(default)] summary: Option<CalculatedText>,
        #[serde(default)] description: Option<CalculatedText>},
    PointBuy{name: String, min: Calculation, max: Calculation, values: Vec<PointBuyRow>,
        cost: Calculation, total: Calculation,#[serde(rename="pointsLeft")] points_left: i64,
        spent: i64, #[serde(default)] ignored: bool},
    Proficiency{name: String,stats: Vec<String>,value: f64},
    PropertySlot{name: String, #[serde(default)] description: Option<CalculatedText>,
        #[serde(rename="slotTags")] slot_tags: Vec<String>,
        #[serde(rename="slotType",default)] slot_type: String,
        #[serde(rename="quantityExpected")] quantity_expected: Calculation,
        #[serde(rename="hideWhenFull")] hide_when_full: bool,
        #[serde(rename="spaceLeft")] space_left: i64,
        #[serde(rename="totalFilled")] total_filled: i64,
        #[serde(rename="extraTags")] extra_tags: Vec<String>,unique: String,
        #[serde(default)] ignored: bool},
    Roll{name: String, #[serde(rename="variableName")] variable_name: String,roll: Calculation},
    SavingThrow{#[serde(default)] name: String,target: String, stat: String, dc: Calculation,
        #[serde(default)] silent: bool},
    Skill{name: String, #[serde(rename="skillType")] skill_type: String,
        #[serde(rename="variableName")] variable_name: String,ability: String,
        #[serde(default)] description: CalculatedText,
        #[serde(rename="abilityMod")] ability_mod: i64,proficiency: f64, value: i64,
        #[serde(rename="baseValue",default)] base_value: Option<Calculation>,
        #[serde(rename="baseProficiency",default)] base_proficiency: Option<f64>},
    SlotFiller{name: String, #[serde(rename="slotQuantityFilled")] slot_quantity_filled: i64,
        description: String, #[serde(default)] picture: String,
        #[serde(rename="slotFillerType",default)] slot_filler_type: Option<String>,
        #[serde(rename="slotFillerCondition",default)] slot_filler_condition: Option<String>},
    SpellList{name: String, #[serde(rename="maxPrepared",default)]max_prepared: Option<Calculation>,
        #[serde(default)] dc: Option<Calculation>,
        #[serde(rename="attackRollBonus",default)] attack_roll_bonus: Option<Calculation>,
        #[serde(default)] description: Option<CalculatedText>,ability: String,
        #[serde(rename="abilityMod",default)] ability_mod: i64},
    Spell{name: String, #[serde(rename="actionType")] action_type: String, target: String,
        resources: Resource, #[serde(rename="castingTime")] casting_time: String,
        duration: String, level: i64, school: String, range: String, verbal: bool,
        somatic: bool, #[serde(default)] material: Option<String>,
        #[serde(default)] concentration: bool, #[serde(default)] ritual: bool,
        #[serde(default)] description: Option<CalculatedText>,
        #[serde(default)] summary: Option<CalculatedText>,prepared: bool,
        #[serde(rename="alwaysPrepared",default)] always_prepared: bool,
        #[serde(rename="deactivatedBySelf",default)] deactivated_by_self: bool},
    Toggle{name: String, #[serde(default)] condition: Option<Calculation>,
        #[serde(rename="showUI",default)] show_ui: bool,
        #[serde(default)] disabled: Option<bool>,#[serde(default)] enabled: Option<bool>,
        #[serde(rename="deactivatedBySelf",default)] deactivated_by_self: bool},
    Trigger{name: String, event: String, timing: String,
        #[serde(rename="targetTags")] target_tags: Vec<String>,
        #[serde(default)] description: Option<CalculatedText>, #[serde(default)] silent: bool,
        #[serde(default)] condition: Option<Calculation>,
        #[serde(rename="actionPropertyType",default)] action_property_type: Option<String>}
}