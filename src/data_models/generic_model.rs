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
    #[serde(default)]
    pub base_value: Option<PropVal>,
    #[serde(default)]
    pub effects: Vec<Effect>
}
#[derive(Serialize,Deserialize,PartialEq,Debug, Default)]
#[serde(rename_all="camelCase")]
pub struct SimpleCalc{
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
    pub inline_calculations: Vec<SimpleCalc>
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
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub quantity: Option<Calculation>,
    #[serde(default)]
    pub item_id: Option<String>
}
#[derive(Serialize,Deserialize,PartialEq,Debug,Default)]
#[serde(rename_all="camelCase")]
pub struct ConsumedResource{
    #[serde(rename="_id")]
    pub id: String,
    #[serde(default)]
    pub variable_name: Option<String>,
    #[serde(default)]
    pub quantity: Option<Calculation>,
    pub available: i64,
    pub stat_name: String,
}
#[derive(Serialize,Deserialize,PartialEq,Debug,Default)]
#[serde(rename_all="camelCase")]
pub struct ExtraTag{
    #[serde(rename="_id")]
    id: String,
    operation: String,
    tags: Vec<String>
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
#[derive(Serialize,Deserialize,PartialEq,Debug,Default)]
#[serde(rename_all="camelCase")]
pub struct PointBuyRow{
    #[serde(rename="_id")]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)] variable_name: Option<String>,
    #[serde(default)] value: i64,
    #[serde(default)] min: Option<Calculation>,
    #[serde(default)] max: Option<Calculation>,
    #[serde(default)] cost: Option<Calculation>, 
    #[serde(default)] spent: i64,
    #[serde(default)] errors: Vec<ParseError>,
}
#[derive(Serialize,Deserialize,PartialEq,Debug)]
#[serde(rename_all="camelCase",tag="attributeType")]
pub enum AttributeType{
    Ability{#[serde(default)] modifier: Option<i64>,#[serde(default)] proficiency: Option<f64>},
    HealthBar{#[serde(rename="healthBarColorMid",default)] health_bar_color_mid: Option<String>,
        #[serde(rename="healthBarColorLow",default)] health_bar_color_low: Option<String>,
        #[serde(rename="healthBarNoDamag",default)] health_bar_no_damage: bool,
        #[serde(rename="healthBarNoHealing",default)] health_bar_no_healing: bool,
        #[serde(rename="healthBarNoDamageOverflow",default)] health_bar_no_damage_overflow: bool,
        #[serde(rename="healthBarNoHealingOverflow",default)] health_bar_no_healing_overflow: bool,
        #[serde(rename="healthBarDamageOrder",default)] health_bar_damage_order: Option<i64>,
        #[serde(rename="healthBarHealingOrder",default)] health_bar_healing_order: Option<i64>,
    },
    HitDice{#[serde(rename="hitDiceSize")] hit_dice_size: String,
        #[serde(rename="constitutionMod",default)] constitution_mod: i64},
    Modifier{},
    Resource{},
    Stat{},
    SpellSlot{#[serde(rename="spellSlotLevel",default)] spell_slot_level: Option<Calculation>},
    Utility{}   
}
impl Default for AttributeType{
    fn default() -> Self {
        Self::Utility {}
    }
}
#[derive(Serialize,Deserialize,PartialEq,Debug)]
#[serde(rename_all="camelCase",tag="type")]
pub enum PropType{
    Action{#[serde(default)] name: String,#[serde(default)] summary: Option<CalculatedText>,
        #[serde(default)] description: Option<CalculatedText>,
        #[serde(rename="actionType")] action_type: String, 
        #[serde(default,rename="variableName")] variable_name: Option<String>,
        target: String,
        #[serde(default,rename="attackRoll")] attack_roll: Option<Calculation>,
        #[serde(default)] uses: Option<Calculation>,
        #[serde(default,rename="usesUsed")] uses_used: Option<i64>,
        #[serde(default)] reset: Option<String>, #[serde(default)] silent: bool,
        resources: Resource,
        #[serde(default,rename="insufficientResources")] insufficient_resources: bool,
        #[serde(default,rename="usesLeft")] uses_left: Option<i64>,
        #[serde(default)] overridden: bool},
    Adjustment{#[serde(default)] amount: Option<Calculation>,target: String,
        #[serde(default)] stat: Option<String>, operation: String,
        #[serde(default)] silent:bool},//attribute damage for some reason
    Attribute{#[serde(default)] name: String, #[serde(rename="variableName")] variable_name: String, 
        #[serde(flatten)] attribute_type: AttributeType,
        #[serde(rename="baseValue",default)] base_value: Option<Calculation>,
        #[serde(default)] description: Option<CalculatedText>, #[serde(default)] damage: Option<i64>,
        #[serde(default)] decimal: bool,
        #[serde(rename="ignoreLowerLimit",default)] ignore_lower_limit: bool,
        #[serde(rename="ignoreUpperLimit",default)] ignore_upper_limit: bool,
        #[serde(rename="hideWhenValueZero",default)] hide_when_value_zero: bool,
        #[serde(rename="hideWhenTotalZero",default)] hide_when_total_zero: bool,
        #[serde(default)] reset:Option<String>,#[serde(default)] total: PropVal,
        #[serde(default)] value: PropVal,#[serde(default)] effects: Vec<Effect>,
        #[serde(default)] hide: bool, #[serde(default)] overridden: bool},
    Branch{#[serde(flatten)] branch_type: BranchType,
        #[serde(default)] text: Option<String>, #[serde(default)] silent: bool},
    Buff{name: String, #[serde(default)] description: Option<CalculatedText>,
        #[serde(rename="hideRemoveButton",default)]hide_remove_button:Option<bool>,
        #[serde(default)] duration: Option<Calculation>, target: String,
        #[serde(default)]silent:Option<bool>,
        #[serde(rename="skipCrystalization",default)]skip_crystalization: Option<bool>,
        #[serde(rename="appliedBy",default)] applied_by: Option<String>},
    BuffRemover{#[serde(default)] name:String,
        #[serde(rename="targetParentBuff",default)]target_parent_buff:bool,
        target: String, #[serde(rename="removeAll",default)]remove_all:bool,
        #[serde(default,rename="targetTags")] target_tags: Vec<String>,
        #[serde(default,rename="extraTags")] extra_tags: Vec<ExtraTag>,
        #[serde(default)]silent:bool},
    Class{#[serde(default)] name: Option<String>, #[serde(default)] description: Option<CalculatedText>,
        #[serde(rename="variableName",default)] variable_name: Option<String>,
        #[serde(default,rename="slotTags")] slot_tags: Vec<String>,
        #[serde(rename="extraTags")] extra_tags: Vec<ExtraTag>,
        #[serde(rename="slotCondition",default)] slot_condition: Option<Calculation>,
        #[serde(default)] level: i64,
        #[serde(rename="missingLevels")] missing_levels: Vec<i64>},
    ClassLevel{#[serde(default)] name: String, #[serde(default)] description: Option<CalculatedText>,
        #[serde(rename="variableName",default)] variable_name: Option<String>,
        level: i64,
        #[serde(rename="slotFillerCondition",default)] slot_filler_condition: Option<Calculation>},
    Constant{#[serde(default)]name: String,
        #[serde(rename="variableName",default)] variable_name: Option<String>,
        #[serde(default)] calculation: Option<String>,
        errors: Vec<ParseError>},
    Container{#[serde(default)] name: String, #[serde(default)] carried: bool,
        #[serde(rename="contentsWeightless",default)] contents_weightless: bool,
        #[serde(default)] weight: f64, #[serde(default)] value: f64,
        #[serde(default)] description: Option<CalculatedText>,
        #[serde(rename="contentsWeight",default)] contents_weight: f64,
        #[serde(rename="carriedWeight",default)] carried_weight: f64,
        #[serde(rename="contentsValue",default)] contents_value: f64,
        #[serde(rename="carriedValue",default)] carried_value: f64},
    Damage{#[serde(default)] amount: Option<Calculation>,
        target: String, #[serde(rename="damageType")] damage_type: String,
        #[serde(default)] silent: bool},
    DamageMultiplier{#[serde(default)] name: String,
        #[serde(rename="damageTypes")] damage_types: Vec<String>,value: f64,
        #[serde(rename="excludeTags",default)] exclude_tags: Vec<String>,
        #[serde(rename="includeTags",default)] include_tags: Vec<String>},
    Effect{#[serde(default)] name: String, operation: String,
        #[serde(default)] amount: Option<Calculation>,
        #[serde(default)] text: Option<CalculatedText>, stats: Vec<String>,
        #[serde(rename="targetByTags",default)] target_by_tags: bool,
        #[serde(rename="targetField",default)] target_field: Option<String>,
        #[serde(rename="targetTags",default)] target_tags: Option<Vec<String>>,
        #[serde(rename="extraTags",default)] extra_tags: Vec<ExtraTag>},
    Feature{#[serde(default)] name: String, #[serde(default)] sumary: Option<CalculatedText>,
        #[serde(default)] description: Option<CalculatedText>},
    Folder{#[serde(default)] name: String, #[serde(rename="groupStats",default)] group_stats: bool,
        #[serde(rename="hideStatsGroup",default)] hide_stats_group: Option<bool>,
        #[serde(default)] tab: Option<String>,#[serde(default)] location: Option<String>},
    Item{#[serde(default)] name: String, #[serde(default)] plural: String,
        #[serde(default)] description: Option<CalculatedText>, quantity: i64,
        #[serde(default)] weight: f64,#[serde(default)] value: f64,
        #[serde(rename="requiresAttunement",default)] requires_attunement: bool,
        #[serde(default)] attuned: Option<bool>,
        #[serde(rename="showIncrement",default)] show_increment: bool, equipped: bool},
    Note{#[serde(default)] name: String, #[serde(default)] summary: Option<CalculatedText>,
        #[serde(default)] description: Option<CalculatedText>},
    PointBuy{#[serde(default)] name: String,#[serde(default)] ignored: bool, values: Vec<PointBuyRow>,
        #[serde(default)] min: Option<Calculation>, #[serde(default)] max: Option<Calculation>,
        #[serde(default)] total: Option<Calculation>,
        cost: Calculation,#[serde(default)] spent: i64,#[serde(rename="pointsLeft")] points_left: i64,
        #[serde(default)]errors: Vec<ParseError>},
    Proficiency{#[serde(default)] name: String,stats: Vec<String>,value: f64},
    PropertySlot{#[serde(default)] name: String, #[serde(default)] description: Option<CalculatedText>,
        #[serde(rename="slotType",default)] slot_type: String,
        #[serde(rename="slotTags")] slot_tags: Vec<String>,
        #[serde(rename="extraTags")] extra_tags: Vec<ExtraTag>,
        #[serde(rename="quantityExpected",default)] quantity_expected: Option<Calculation>,
        #[serde(default)] ignored: bool,
        #[serde(rename="slotCondition",default)] slot_condition: Option<Calculation>,
        #[serde(rename="hideWhenFull")] hide_when_full: bool,unique: String,
        #[serde(rename="totalFilled",default)] total_filled: i64,
        #[serde(rename="spaceLeft",default)] space_left: Option<i64>},
    Roll{#[serde(default)] name: String, #[serde(rename="variableName")] variable_name: String,
        #[serde(default)] roll: Option<Calculation>,#[serde(default)] silent: bool},
    SavingThrow{#[serde(default)] name: String, #[serde(default)] dc: Option<Calculation>,
        target: String, #[serde(default)] stat: Option<String>,
        #[serde(default)] silent: bool},
    Skill{#[serde(default)] name: String,
        #[serde(rename="variableName",default)] variable_name: Option<String>,
        #[serde(default)] ability: Option<String>,#[serde(rename="skillType")] skill_type: String,
        #[serde(rename="baseProficiency",default)] base_proficiency: Option<f64>,
        #[serde(rename="baseValue",default)] base_value: Option<Calculation>,
        #[serde(default)] description: Option<CalculatedText>, value: i64,
        #[serde(rename="abilityMod",default)] ability_mod: i64,
        #[serde(default)] advantage: i64, #[serde(default,rename="passiveBonus")] passive_bonus: i64, proficiency: f64,
        #[serde(default,rename="conditionalBenefits")] conditional_benifits: Vec<String>,
        #[serde(default)] fail: i64, #[serde(default)] hide:bool, #[serde(default)] overridden: bool,
        #[serde(default)] effects: Vec<Effect>
        },
    SlotFiller{#[serde(default)] name: String, #[serde(default)] picture: String,
        #[serde(default)] description: String, 
        #[serde(rename="slotFillerType",default)] slot_filler_type: Option<String>,
        #[serde(rename="slotQuantityFilled")] slot_quantity_filled: i64,
        #[serde(rename="slotFillerCondition",default)] slot_filler_condition: Option<String>},
    SpellList{#[serde(default)] name: String,
        #[serde(rename="maxPrepared",default)]max_prepared: Option<Calculation>,
        #[serde(default)] dc: Option<Calculation>,
        #[serde(rename="attackRollBonus",default)] attack_roll_bonus: Option<Calculation>,
        #[serde(default)] description: Option<CalculatedText>,ability: String,
        #[serde(rename="abilityMod",default)] ability_mod: i64},
    Spell{#[serde(default)] name: String,
        #[serde(rename="alwaysPrepared",default)] always_prepared: bool,
        #[serde(default)] prepared: bool,
        #[serde(rename="castWithoutSpellSlots",default)] cast_without_spell_slots: bool,
        #[serde(rename="hasAttackRoll")] has_attack_roll: bool,
        #[serde(rename="castingTime",default)] casting_time: Option<String>,
        #[serde(default)] range: Option<String>,#[serde(default)] duration: Option<String>,
        #[serde(default)] verbal: bool, #[serde(default)] somatic: bool,
        #[serde(default)] concentration: bool, #[serde(default)] material: Option<String>,
        #[serde(default)] ritual: bool, level: i64, school: String,
        #[serde(default)] summary: Option<CalculatedText>,
        #[serde(default)] description: Option<CalculatedText>,
        #[serde(rename="actionType")] action_type: String, 
        #[serde(default,rename="variableName")] variable_name: Option<String>,
        target: String,
        #[serde(default,rename="attackRoll")] attack_roll: Option<Calculation>,
        #[serde(default)] uses: Option<Calculation>,
        #[serde(default,rename="usesUsed")] uses_used: Option<i64>,
        #[serde(default)] reset: Option<String>, #[serde(default)] silent: bool,
        resources: Resource,
        #[serde(default,rename="insufficientResources")] insufficient_resources: bool,
        #[serde(default,rename="usesLeft")] uses_left: Option<i64>,
        #[serde(default)] overridden: bool,
        #[serde(rename="deactivatedBySelf",default)] deactivated_by_self: bool},
    Toggle{#[serde(default)] name: String,
        #[serde(default,rename="variableName")] variable_name: Option<String>,
        #[serde(rename="showUI",default)] show_ui: bool,
        #[serde(default)] disabled: Option<bool>,#[serde(default)] enabled: Option<bool>,
        #[serde(default)] condition: Option<Calculation>,
        #[serde(rename="deactivatedBySelf",default)] deactivated_by_self: bool},
    Trigger{#[serde(default)] name: String,
        #[serde(default)] description: Option<CalculatedText>,
        event: String,
        #[serde(rename="actionPropertyType",default)] action_property_type: Option<String>,
        timing: String,#[serde(default)] condition: Option<Calculation>,
        #[serde(rename="targetTags")] target_tags: Vec<String>, 
        #[serde(rename="extraTags")] extra_tags: Vec<ExtraTag>,
        #[serde(default)] silent: bool}
}
impl Default for PropType{
    fn default() -> Self {
        Self::Folder { name: String::default(), group_stats: false, hide_stats_group: None, location: None, tab: None }
    }
}