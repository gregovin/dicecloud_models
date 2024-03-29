use core::fmt;
use std::default::{Default};
use serde::{Serialize,Deserialize};



#[derive(Serialize, Deserialize,PartialEq,Eq,Debug,Default,Hash,Clone,Copy)]
/// A structure to store the death save info as in the creature settings
pub struct DeathSaveInfo{
    pub pass: usize,
    pub fail: usize,
    #[serde(rename = "canDeathSave")]
    pub can_death_save: bool,
    pub stable: bool
}
/// A structure to store the denormalized stats(such as xp and milestone levels) as in the creature settings
#[derive(Serialize,Deserialize, PartialEq,Eq,Debug,Default,Hash,Clone,Copy,PartialOrd, Ord)]
pub struct DenormalizedStats{
    #[serde(rename = "milestoneLevels")]
    pub milestone_levels: usize,
    pub xp: usize,
}
/// A structure to deal with a character's advanced settings, such as wether to show the tree tab
#[derive(Serialize,Deserialize,Default, PartialEq,Debug,Clone)]
#[serde(rename_all="camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub struct Settings{
    #[serde(default)]
    pub show_tree_tab: bool,
    #[serde(default)]
    pub hide_rest_buttons: bool,
    #[serde(default)]
    pub hide_unused_stats: bool,
    #[serde(default)]
    pub hide_spells_tab: bool,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub hit_dice_reset_multiplier: Option<f64>,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub discord_webhook: Option<String>
}
/// A structure to deal with a character's overall settings
#[derive(Serialize, Deserialize,PartialEq,Debug,Default,Clone)]
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
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub picture: Option<String>,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub avatar_picture: Option<String>
}
#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(untagged,rename_all="camelCase")]
/// A structure that represents a value an attribute or constant can take
/// 
/// #Examples
/// ```
/// # use std::error::Error;
/// # 
/// # fn main() -> Result<(), Box<dyn Error>> {
/// use dicecloud_models::data_models::generic_model::PropVal;
/// 
/// let first: PropVal = serde_json::from_str("false")?;
/// assert_eq!(first,PropVal::Boolean(false));
/// let second: PropVal = serde_json::from_str("10.6")?;
/// assert_eq!(second,PropVal::Fraction(10.6));
/// let third: PropVal = serde_json::from_str("\"test\"")?;
/// assert_eq!(third,PropVal::Str("test".to_string()));
/// # Ok(())
/// # }
/// ```
pub enum PropVal{
    Boolean(bool),
    ///This variant should always be `PropVal::None(None)`, though if it is `PropVal::Some(())` reencoding won't break anything
    None(Option<()>),
    Number(i64),
    Fraction(f64),
    Str(String),
}
impl PartialEq for PropVal{
    fn eq(&self, other: &Self) -> bool {
        match  (&self,other) {
            (PropVal::Boolean(b1),PropVal::Boolean(b2))=>b1.eq(b2),
            (PropVal::None(_),PropVal::None(_))=>true,
            (PropVal::Number(n1),PropVal::Number(n2))=>n1.eq(n2),
            (PropVal::Fraction(f1),PropVal::Fraction(f2))=>f1.eq(f2),
            (PropVal::Str(s1),PropVal::Str(s2))=>s1.eq(s2),
            (_,_)=>false
        }
    }
}
impl Default for PropVal{
    fn default()->Self{
        Self::Number(0)
    }
}
impl PropVal{
    /// Gets the value as a bool, if it is a bool
    /// 
    /// #Example
    /// ```
    /// use dicecloud_models::data_models::generic_model::PropVal;
    /// let test=PropVal::Boolean(true);
    /// assert_eq!(test.as_bool(),Some(true));
    /// let test2=PropVal::Number(10);
    /// assert_eq!(test2.as_bool(),None);
    /// ```
    #[must_use]
    pub const fn as_bool(&self)->Option<bool>{
        match self{
            Self::Boolean(b)=>Some(*b),
            _=>None
        }
    }
    /// Gets the value as an integer, if it is an integer
    /// 
    /// #Example
    /// ```
    /// use dicecloud_models::data_models::generic_model::PropVal;
    /// let test=PropVal::Number(10);
    /// assert_eq!(test.as_i64(),Some(10));
    /// let test2=PropVal::Boolean(true);
    /// assert_eq!(test2.as_i64(),None);
    /// ```
    #[must_use]
    pub const fn as_i64(&self)->Option<i64>{
        match self{
            Self::Number(k)=>Some(*k),
            _=>None
        }
    }
    /// Gets the value as a floating point number, if it is number-like
    /// 
    /// #Example
    /// ```
    /// use dicecloud_models::data_models::generic_model::PropVal;
    /// let test=PropVal::Number(10);
    /// assert_eq!(test.as_f64(),Some(10.0));
    /// let test2=PropVal::Fraction(2.3);
    /// assert_eq!(test2.as_f64(),Some(2.3));
    /// let test3=PropVal::Str("test".to_string());
    /// assert_eq!(test3.as_f64(),None);
    /// ```
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub const fn as_f64(&self)->Option<f64>{
        match self{
            Self::Number(k) => Some(*k as f64),
            Self::Fraction(f) => Some(*f),
            _=>None
        }
    }
}
impl fmt::Display for PropVal{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::Boolean(b) => write!(f,"{b}"),
            Self::Fraction(n) => write!(f,"{n}"),
            Self::Number(k)=>write!(f,"{k}"),
            Self::Str(s)=>write!(f,"{s}"),
            Self::None(_)=>write!(f,"null")
        }
    }
}
/// Wraps a value in a structure (relevant for effects)
#[derive(Serialize,Deserialize,PartialEq,Debug,Default,Clone)]
pub struct ValWrap{
    pub value: PropVal
}
/// Encapsulates a `ParseNode` for calculations.
/// 
/// You probably don't need to look at this
#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
#[serde(rename_all="camelCase",tag="parseType")]
pub enum ParseNode{
    Accessor{path: Vec<String>,name: String},
    Array{values: Vec<Box<ParseNode>>},
    Call{#[serde(rename="functionName")]function_name: String,args: Vec<Box<ParseNode>>},
    Error{error: ParseError},
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
    fn default()->Self{
        Self::Constant{value_type:"number".to_string(),value: PropVal::default()}
    }
}
#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Default,Hash,Clone)]
/// Encapsulates a `ParseError` for calculations
/// 
/// You probably don't need to look at this
pub struct ParseError{
    #[serde(rename="type")]
    pub typ: String,
    pub message: String,
}
/// Represents a Calculation as in a calculated field
/// 
/// The value field is probably what you want to look at
/// ```
#[derive(Serialize,Deserialize,PartialEq,Debug, Default,Clone)]
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
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub base_value: Option<PropVal>,
    #[serde(default)]
    pub effects: Vec<Effect>
}
/// A simpler version of the calculation type for calculated text fields.
/// 
/// You probably don't need this
#[derive(Serialize,Deserialize,PartialEq,Debug, Default,Clone)]
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
/// Represents an identifier (ie for parent and ancestors fields)
#[derive(Serialize,Deserialize,PartialEq,Eq,Debug, Default,Hash,Clone)]
#[serde(rename_all="camelCase")]
pub struct Identifier{
    pub id: String,
    pub collection: String,
}
/// Represents a calculated text field, such as descriptions
/// 
/// value is the final displayed text, text is the initial text with inline calculations
#[derive(Serialize,Deserialize,PartialEq,Debug, Default,Clone)]
#[serde(rename_all="camelCase")]
pub struct CalculatedText{
    pub text: String,
    pub value: String,
    pub hash: i64,
    pub inline_calculations: Vec<SimpleCalc>
}
/// Represents an effect on an attribute(not an effect property)
#[derive(Serialize,Deserialize,PartialEq,Debug, Default,Clone)]
#[serde(rename_all="camelCase")]
pub struct Effect{
    #[serde(rename="_id")]
    pub id: String,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
    pub operation: String,
    #[serde(default)]
    pub amount:ValWrap,
    #[serde(default,rename="type",skip_serializing_if="Option::is_none")]
    pub typ: Option<String>,
}
/// Represents amunition as used by actions
#[derive(Serialize,Deserialize,PartialEq,Debug,Default,Clone)]
#[serde(rename_all="camelCase")]
pub struct ConsumedItem{
    #[serde(rename="_id")]
    pub id: String,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub tag: Option<String>,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub quantity: Option<Calculation>,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub item_name: Option<String>
}
/// Represents a resource consumed by an action
#[derive(Serialize,Deserialize,PartialEq,Debug,Default,Clone)]
#[serde(rename_all="camelCase")]
pub struct ConsumedResource{
    #[serde(rename="_id")]
    pub id: String,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub variable_name: Option<String>,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub quantity: Option<Calculation>,
    #[serde(default)]
    pub available: i64,
    #[serde(default)]
    pub stat_name: Option<String>,
}
/// Represents the `ExtraTags` field, which is used by slot like properties for tag logic
#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Default,Clone,Hash)]
#[serde(rename_all="camelCase")]
pub struct ExtraTag{
    #[serde(rename="_id")]
    pub id: String,
    pub operation: String,
    pub tags: Vec<String>
}
/// Represents the total resources consumed by an action(including attributes and items)
#[derive(Serialize,Deserialize,PartialEq,Debug,Default,Clone)]
#[serde(rename_all="camelCase")]
pub struct Resource{
    pub items_consumed: Vec<ConsumedItem>,
    pub attributes_consumed: Vec<ConsumedResource>,
}
/// Represents an Icon in the api
/// 
/// Note that shape is just the raw svg
#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Default,Clone,Hash)]
#[serde(rename_all="camelCase")]
pub struct Icon{
    pub name: String,
    pub shape: String,
}
/// Enum that describes the type of branch for a branch property
#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
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
/// Represents a row for a point buy property
#[derive(Serialize,Deserialize,PartialEq,Debug,Default,Clone)]
#[serde(rename_all="camelCase")]
pub struct PointBuyRow{
    #[serde(rename="_id")]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default,skip_serializing_if="Option::is_none")]
    variable_name: Option<String>,
    #[serde(default)] value: i64,
    #[serde(default)] spent: i64,
    #[serde(default)] errors: Vec<ParseError>,
}
/// Enum describes the types of attributes and assosiated properties
#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
#[serde(rename_all="camelCase",tag="attributeType")]
pub enum AttributeType{
    Ability{#[serde(default)] modifier: i64,
        #[serde(default,skip_serializing_if="Option::is_none")] proficiency: Option<f64>},
    HealthBar{#[serde(rename="healthBarColorMid",default,skip_serializing_if="Option::is_none")]
        health_bar_color_mid: Option<String>,
        #[serde(rename="healthBarNoDamage",default)] health_bar_no_damage: bool,
        #[serde(rename="healthBarNoHealing",default)] health_bar_no_healing: bool,
        #[serde(rename="healthBarNoDamageOverflow",default)] health_bar_no_damage_overflow: bool,
        #[serde(rename="healthBarNoHealingOverflow",default)] health_bar_no_healing_overflow: bool,
        #[serde(rename="healthBarDamageOrder",default,skip_serializing_if="Option::is_none")] health_bar_damage_order: Option<i64>,
        #[serde(rename="healthBarHealingOrder",default,skip_serializing_if="Option::is_none")] health_bar_healing_order: Option<i64>,
    },
    HitDice{#[serde(rename="hitDiceSize")] hit_dice_size: String,
        #[serde(rename="constitutionMod",default)] constitution_mod: i64},
    Modifier{},
    Resource{},
    Stat{},
    SpellSlot{#[serde(rename="spellSlotLevel",default,skip_serializing_if="Option::is_none")] spell_slot_level: Option<Calculation>},
    Utility{}   
}
impl Default for AttributeType{
    fn default() -> Self {
        Self::Utility {}
    }
}
/// Represents a cache for errors on ref properties
/// 
/// Don't worry about this
#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Default,Clone,Hash)]
pub struct Cache{
    pub error: String
}
#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
#[serde(rename_all="camelCase",tag="type")]
/// an enum that encodes the various property types
/// 
pub enum PropType{
    Action{#[serde(default)] name: String,
        #[serde(default,skip_serializing_if="Option::is_none")]summary: Option<CalculatedText>,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>,
        #[serde(rename="actionType")] action_type: String, 
        #[serde(default,rename="variableName",skip_serializing_if="Option::is_none")] variable_name: Option<String>,
        target: String,
        #[serde(default,rename="attackRoll",skip_serializing_if="Option::is_none")] attack_roll: Option<Calculation>,
        #[serde(default,skip_serializing_if="Option::is_none")] uses: Option<Calculation>,
        #[serde(default,rename="usesUsed")] uses_used: i64,
        #[serde(default,skip_serializing_if="Option::is_none")] reset: Option<String>,
        #[serde(default)] silent: bool,
        resources: Resource,
        #[serde(default,rename="insufficientResources")] insufficient_resources: bool,
        #[serde(default,rename="usesLeft",skip_serializing_if="Option::is_none")] uses_left: Option<i64>,
        #[serde(default)] overridden: bool},
    Adjustment{#[serde(default,skip_serializing_if="Option::is_none")] amount: Option<Calculation>,
        target: String,
        #[serde(default,skip_serializing_if="Option::is_none")] stat: Option<String>, operation: String,
        #[serde(default)] silent:bool},//attribute damage for some reason
    Attribute{#[serde(default)] name: String, #[serde(rename="variableName")] variable_name: String, 
        #[serde(flatten)] attribute_type: AttributeType,
        #[serde(rename="baseValue",default,skip_serializing_if="Option::is_none")] base_value: Option<Calculation>,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>,
        #[serde(default)] damage: i64,
        #[serde(default)] decimal: bool,
        #[serde(rename="ignoreLowerLimit",default)] ignore_lower_limit: bool,
        #[serde(rename="ignoreUpperLimit",default)] ignore_upper_limit: bool,
        #[serde(rename="hideWhenValueZero",default)] hide_when_value_zero: bool,
        #[serde(rename="hideWhenTotalZero",default)] hide_when_total_zero: bool,
        #[serde(default,skip_serializing_if="Option::is_none")] reset:Option<String>,
        #[serde(default)] total: PropVal,
        #[serde(default)] value: PropVal,#[serde(default)] effects: Vec<Effect>,
        #[serde(default)] hide: bool, #[serde(default)] overridden: bool},
    Branch{#[serde(flatten)] branch_type: BranchType,
        #[serde(default,skip_serializing_if="Option::is_none")] text: Option<String>,
        #[serde(default)] silent: bool},
    Buff{name: String,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>,
        #[serde(rename="hideRemoveButton",default)]hide_remove_button:bool,
        #[serde(default,skip_serializing_if="Option::is_none")] duration: Option<Calculation>,
        target: String,
        #[serde(default)] silent:bool,
        #[serde(rename="skipCrystalization",default)]skip_crystalization: bool,
        #[serde(rename="appliedBy",default,skip_serializing_if="Option::is_none")] applied_by: Option<String>},
    BuffRemover{#[serde(default)] name:String,
        #[serde(rename="targetParentBuff",default)]target_parent_buff:bool,
        target: String, #[serde(rename="removeAll",default)]remove_all:bool,
        #[serde(default,rename="targetTags")] target_tags: Vec<String>,
        #[serde(default,rename="extraTags")] extra_tags: Vec<ExtraTag>,
        #[serde(default)]silent:bool},
    Class{#[serde(default)] name: String,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>,
        #[serde(rename="variableName",default,skip_serializing_if="Option::is_none")] variable_name: Option<String>,
        #[serde(default,rename="slotTags")] slot_tags: Vec<String>,
        #[serde(rename="extraTags",default)] extra_tags: Vec<ExtraTag>,
        #[serde(rename="slotCondition",default,skip_serializing_if="Option::is_none")] slot_condition: Option<Calculation>,
        #[serde(default)] level: i64,
        #[serde(rename="missingLevels",default)] missing_levels: Vec<i64>},
    ClassLevel{#[serde(default)] name: String,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>,
        #[serde(rename="variableName",default,skip_serializing_if="Option::is_none")] variable_name: Option<String>,
        level: i64,
        #[serde(rename="slotFillerCondition",default,skip_serializing_if="Option::is_none")] slot_filler_condition: Option<Calculation>},
    Constant{#[serde(default)]name: String,
        #[serde(rename="variableName",default,skip_serializing_if="Option::is_none")] variable_name: Option<String>,
        #[serde(default,skip_serializing_if="Option::is_none")] calculation: Option<String>,
        errors: Vec<ParseError>},
    Container{#[serde(default)] name: String, #[serde(default)] carried: bool,
        #[serde(rename="contentsWeightless",default)] contents_weightless: bool,
        #[serde(default)] weight: f64, #[serde(default)] value: f64,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>,
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
        #[serde(default,skip_serializing_if="Option::is_none")] amount: Option<Calculation>,
        #[serde(default,skip_serializing_if="Option::is_none")] text: Option<String>,
        stats: Vec<String>,
        #[serde(rename="targetByTags",default)] target_by_tags: bool,
        #[serde(rename="targetField",default,skip_serializing_if="Option::is_none")] target_field: Option<String>,
        #[serde(rename="targetTags",default,skip_serializing_if="Option::is_none")] target_tags: Option<Vec<String>>,
        #[serde(rename="extraTags",default)] extra_tags: Vec<ExtraTag>},
    Feature{#[serde(default)] name: String, #[serde(default)] sumary: Option<CalculatedText>,
        #[serde(default)] description: Option<CalculatedText>},
    Folder{#[serde(default)] name: String, #[serde(rename="groupStats",default)] group_stats: bool,
        #[serde(rename="hideStatsGroup",default,skip_serializing_if="Option::is_none")] hide_stats_group: Option<bool>,
        #[serde(default,skip_serializing_if="Option::is_none")] tab: Option<String>,
        #[serde(default,skip_serializing_if="Option::is_none")] location: Option<String>},
    Item{#[serde(default)] name: String, #[serde(default)] plural: String,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>,
        quantity: i64,
        #[serde(default)] weight: f64,#[serde(default)] value: f64,
        #[serde(rename="requiresAttunement",default)] requires_attunement: bool,
        #[serde(default,skip_serializing_if="Option::is_none")] attuned: Option<bool>,
        #[serde(rename="showIncrement",default)] show_increment: bool, equipped: bool},
    Note{#[serde(default)] name: String, 
        #[serde(default,skip_serializing_if="Option::is_none")] summary: Option<CalculatedText>,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>},
    PointBuy{#[serde(default)] name: String,#[serde(default)] ignored: bool, values: Vec<PointBuyRow>,
        #[serde(default,skip_serializing_if="Option::is_none")] min: Option<Calculation>,
        #[serde(default,skip_serializing_if="Option::is_none")] max: Option<Calculation>,
        #[serde(default,skip_serializing_if="Option::is_none")] total: Option<Calculation>,
        cost: Calculation,#[serde(default)] spent: i64,#[serde(rename="pointsLeft")] points_left: i64,
        #[serde(default)]errors: Vec<ParseError>},
    Proficiency{#[serde(default)] name: String,stats: Vec<String>,value: f64},
    PropertySlot{#[serde(default)] name: String,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>,
        #[serde(rename="slotType",default,skip_serializing_if="Option::is_none")] slot_type: Option<String>,
        #[serde(rename="slotTags")] slot_tags: Vec<String>,
        #[serde(rename="extraTags")] extra_tags: Vec<ExtraTag>,
        #[serde(rename="quantityExpected",default,skip_serializing_if="Option::is_none")] quantity_expected: Option<Calculation>,
        #[serde(default)] ignored: bool,
        #[serde(rename="slotCondition",default,skip_serializing_if="Option::is_none")] slot_condition: Option<Calculation>,
        #[serde(rename="hideWhenFull")] hide_when_full: bool,unique: String,
        #[serde(rename="totalFilled",default)] total_filled: i64,
        #[serde(rename="spaceLeft",default,skip_serializing_if="Option::is_none")] space_left: Option<i64>},
    Reference{#[serde(rename="ref")] refer: Identifier,cache: Cache},
    Roll{#[serde(default)] name: String, #[serde(rename="variableName")] variable_name: String,
        #[serde(default,skip_serializing_if="Option::is_none")] roll: Option<Calculation>,
        #[serde(default)] silent: bool},
    SavingThrow{#[serde(default)] name: String,
        #[serde(default,skip_serializing_if="Option::is_none")] dc: Option<Calculation>,
        target: String, #[serde(default,skip_serializing_if="Option::is_none")] stat: Option<String>,
        #[serde(default)] silent: bool},
    Skill{#[serde(default)] name: String,
        #[serde(rename="variableName",default,skip_serializing_if="Option::is_none")] variable_name: Option<String>,
        #[serde(default,skip_serializing_if="Option::is_none")] ability: Option<String>,
        #[serde(rename="skillType")] skill_type: String,
        #[serde(rename="baseProficiency",default,skip_serializing_if="Option::is_none")] base_proficiency: Option<f64>,
        #[serde(rename="baseValue",default,skip_serializing_if="Option::is_none")] base_value: Option<Calculation>,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>,
        value: i64,
        #[serde(rename="abilityMod",default)] ability_mod: i64,
        #[serde(default)] advantage: i64, #[serde(default,rename="passiveBonus")] passive_bonus: i64,
        proficiency: f64,
        #[serde(default,rename="conditionalBenefits")] conditional_benifits: Vec<String>,
        #[serde(default)] fail: i64, #[serde(default)] hide:bool, #[serde(default)] overridden: bool,
        #[serde(default)] effects: Vec<Effect>
        },
    SlotFiller{#[serde(default)] name: String,
        #[serde(default,skip_serializing_if="Option::is_none")] picture: Option<String>,
        #[serde(default)] description: String, 
        #[serde(rename="slotFillerType",default,skip_serializing_if="Option::is_none")] slot_filler_type: Option<String>,
        #[serde(rename="slotQuantityFilled")] slot_quantity_filled: i64,
        #[serde(rename="slotFillerCondition",default,skip_serializing_if="Option::is_none")] slot_filler_condition: Option<String>},
    SpellList{#[serde(default)] name: String,
        #[serde(rename="maxPrepared",default,skip_serializing_if="Option::is_none")]max_prepared: Option<Calculation>,
        #[serde(default,skip_serializing_if="Option::is_none")] dc: Option<Calculation>,
        #[serde(rename="attackRollBonus",default,skip_serializing_if="Option::is_none")] attack_roll_bonus: Option<Calculation>,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>,
        ability: String,
        #[serde(rename="abilityMod",default)] ability_mod: i64},
    Spell{#[serde(default)] name: String,
        #[serde(rename="alwaysPrepared",default)] always_prepared: bool,
        #[serde(default)] prepared: bool,
        #[serde(rename="castWithoutSpellSlots",default)] cast_without_spell_slots: bool,
        #[serde(rename="hasAttackRoll",default)] has_attack_roll: bool,
        #[serde(rename="castingTime",default,skip_serializing_if="Option::is_none")] casting_time: Option<String>,
        #[serde(default,skip_serializing_if="Option::is_none")] range: Option<String>,
        #[serde(default,skip_serializing_if="Option::is_none")] duration: Option<String>,
        #[serde(default)] verbal: bool, #[serde(default)] somatic: bool,
        #[serde(default)] concentration: bool,
        #[serde(default,skip_serializing_if="Option::is_none")] material: Option<String>,
        #[serde(default)] ritual: bool, level: i64, school: String,
        #[serde(default,skip_serializing_if="Option::is_none")] summary: Option<CalculatedText>,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>,
        #[serde(rename="actionType")] action_type: String, 
        #[serde(default,rename="variableName",skip_serializing_if="Option::is_none")] variable_name: Option<String>,
        target: String,
        #[serde(default,rename="attackRoll",skip_serializing_if="Option::is_none")] attack_roll: Option<Calculation>,
        #[serde(default,skip_serializing_if="Option::is_none")] uses: Option<Calculation>,
        #[serde(default,rename="usesUsed")] uses_used: Option<i64>,
        #[serde(default,skip_serializing_if="Option::is_none")] reset: Option<String>,
        #[serde(default)] silent: bool,
        resources: Resource,
        #[serde(default,rename="insufficientResources")] insufficient_resources: bool,
        #[serde(default,rename="usesLeft",skip_serializing_if="Option::is_none")] uses_left: Option<i64>,
        #[serde(default)] overridden: bool,
        #[serde(rename="deactivatedBySelf",default)] deactivated_by_self: bool},
    Toggle{#[serde(default)] name: String,
        #[serde(default,rename="variableName",skip_serializing_if="Option::is_none")] variable_name: Option<String>,
        #[serde(rename="showUI",default)] show_ui: bool,
        #[serde(default,skip_serializing_if="Option::is_none")] disabled: Option<bool>,
        #[serde(default,skip_serializing_if="Option::is_none")] enabled: Option<bool>,
        #[serde(default,skip_serializing_if="Option::is_none")] condition: Option<Calculation>,
        #[serde(rename="deactivatedBySelf",default)] deactivated_by_self: bool},
    Trigger{#[serde(default)] name: String,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>,
        event: String,
        #[serde(rename="actionPropertyType",default,skip_serializing_if="Option::is_none")] action_property_type: Option<String>,
        timing: String,
        #[serde(default,skip_serializing_if="Option::is_none")] condition: Option<Calculation>,
        #[serde(rename="targetTags",default)] target_tags: Vec<String>, 
        #[serde(rename="extraTags",default)] extra_tags: Vec<ExtraTag>,
        #[serde(default)] silent: bool}
}
impl Default for PropType{
    fn default() -> Self {
        Self::Folder { name: String::default(), group_stats: false, hide_stats_group: None, location: None, tab: None }
    }
}
/// Enum representing different types of variables in the `creatureVariables` field
#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
#[serde(rename_all="camelCase",tag="type")]
pub enum VariableType{
    Attribute{#[serde(rename="baseValue",default,skip_serializing_if="Option::is_none")] base_value: Option<Calculation>,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>,
        #[serde(default,skip_serializing_if="Option::is_none")] damage: Option<i64>,
        #[serde(default)] decimal: bool,
        #[serde(rename="ignoreLowerLimit",default)] ignore_lower_limit: bool,
        #[serde(rename="ignoreUpperLimit",default)] ignore_upper_limit: bool,
        #[serde(rename="hideWhenValueZero",default)] hide_when_value_zero: bool,
        #[serde(rename="hideWhenTotalZero",default)] hide_when_total_zero: bool,
        #[serde(default,skip_serializing_if="Option::is_none")] reset:Option<String>,
        #[serde(default)] total: PropVal,
        #[serde(default)] value: PropVal,#[serde(default)] effects: Vec<Effect>,
        #[serde(default)] hide: bool, #[serde(default)] overridden: bool},
    Constant{#[serde(default,skip_serializing_if="Option::is_none")] calculation: Option<String>,
        value: ParseNode,
        errors: Vec<ParseError>},
    Class{#[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>,
        #[serde(default,rename="slotTags")] slot_tags: Vec<String>,
        #[serde(rename="extraTags")] extra_tags: Vec<ExtraTag>,
        #[serde(rename="slotCondition",default,skip_serializing_if="Option::is_none")] slot_condition: Option<Calculation>,
        #[serde(default)] level: i64,
        #[serde(rename="missingLevels",default)] missing_levels: Vec<i64>},
    Skill{
        #[serde(default,skip_serializing_if="Option::is_none")] ability: Option<String>,
        #[serde(rename="skillType")] skill_type: String,
        #[serde(rename="baseProficiency",default,skip_serializing_if="Option::is_none")] base_proficiency: Option<f64>,
        #[serde(rename="baseValue",default,skip_serializing_if="Option::is_none")] base_value: Option<Calculation>,
        #[serde(default,skip_serializing_if="Option::is_none")] description: Option<CalculatedText>, 
        value: i64,
        #[serde(rename="abilityMod",default)] ability_mod: i64,
        #[serde(default)] advantage: i64,
        #[serde(default,rename="passiveBonus")] passive_bonus: i64, proficiency: f64,
        #[serde(default,rename="conditionalBenefits")] conditional_benifits: Vec<String>,
        #[serde(default)] fail: i64, #[serde(default)] hide:bool, #[serde(default)] overridden: bool,
        #[serde(default)] effects: Vec<Effect>},
    Toggle{#[serde(rename="showUI",default)] show_ui: bool,
        #[serde(default,skip_serializing_if="Option::is_none")] disabled: Option<bool>,
        #[serde(default,skip_serializing_if="Option::is_none")] enabled: Option<bool>,
        #[serde(default,skip_serializing_if="Option::is_none")] condition: Option<Calculation>,
        #[serde(rename="deactivatedBySelf",default)] deactivated_by_self: bool},
}
impl Eq for VariableType{}
/// Represents a variable in the `creatureVariables` field
#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Clone)]
#[serde(rename_all="camelCase")]
pub struct GenericVariable{
    #[serde(rename="_id")] id: String,
    variable_name: String,
    #[serde(default)] name: String,
    order: i64,
    #[serde(flatten)] var_type: VariableType,
    #[serde(default)] library_tags: Vec<String>,
    #[serde(default,skip_serializing_if="Option::is_none")] library_node_id: Option<String>,
}
/// represents a thing in the `creatureVariables` field
#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Clone)]
#[serde(rename_all="camelCase",untagged)]
pub enum CharacterVar{
    Empty{#[serde(skip_serializing_if="Option::is_none",default)] v: Option<i64>},
    Str(String),
    Var(GenericVariable)
}
impl Default for CharacterVar{
    fn default() -> Self {
        Self::Empty{v: None}
    }
}