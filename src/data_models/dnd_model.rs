use std::cmp::{PartialOrd,Ordering,Ord};
use std::collections::HashMap;
use crate::data_models::{flat_model::*,generic_model::*};
use std::fmt::{self,Write};
///defines an ability score by the value(score) and name
#[derive(Clone,Eq,PartialEq,Hash,Debug,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AbilityScore{
    score: i64,
    name: String,
}
impl AbilityScore{
    pub fn score(&self)->i64{
        self.score
    }
    pub fn name(&self)->&String{
        &self.name
    }
    /// get an ability score's modifier
    /// #example
    /// ```
    /// use dicecloud_sheet_printer::holding_structs::{AbilityScore};
    ///
    /// let sten = AbilityScore::new("Strength".to_string(),14);
    /// let con = AbilityScore::new("Constitution".to_string(),11);
    /// let dex = AbilityScore::new("Dexterity".to_string(),9);
    /// assert_eq!(sten.modifier(),2);
    /// assert_eq!(con.modifier(),0);
    /// assert_eq!(dex.modifier(),-1);
    /// ```
    pub fn modifier(&self)->i64{
        self.score/2 -5
    }
    pub fn new(name: String, score: i64)->AbilityScore{
        AbilityScore { score, name}
    }
}
///Types of proficiency listed
#[derive(Debug, Eq, PartialEq,PartialOrd,Ord,Clone,Hash,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Proficiency {
    #[default]
    None,
    Half,
    Profficient,
    Expert,
}
///A skill is a bonus, name, and prof
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Skill{
    bonus: i64,
    name: String,
    prof_rank: Proficiency
}
impl Skill{
    pub fn prof(&self)->&Proficiency{
        &self.prof_rank
    }
    pub fn modifier(&self)->i64{
        self.bonus
    }
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn new(name: String, bonus: i64, prof_rank: Proficiency)->Skill{
        Skill {bonus, name, prof_rank}
    }
}
impl PartialOrd for Skill{
    fn partial_cmp(&self, other: &Skill)->Option<Ordering>{
        if self.name()!=other.name(){
            return self.name().partial_cmp(other.name());
        }
        if self.prof()!=other.prof() {
            return self.prof().partial_cmp(other.prof());
        }
        self.modifier().partial_cmp(&other.modifier())
    }
}
impl Ord for Skill{
    fn cmp(&self, other: &Skill)->Ordering{
        self.partial_cmp(other).unwrap()
    }
}
///A class is a name and a level
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Class{
    name: String,
    level: i64,
    pub start_class: bool,
}
impl Class{
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn level(&self)->i64{
        self.level
    }
    pub fn new(name: String, level: i64)->Class{
        Class { name, level, start_class: false}
    }
}
impl PartialOrd for Class{
    fn partial_cmp(&self, other: &Class) -> Option<Ordering> {
        if self.start_class != other.start_class {
            if self.start_class && !other.start_class {Some(Ordering::Less)} else {Some(Ordering::Greater)}
        } else if self.level!=other.level(){
            other.level().partial_cmp(&self.level)
        } else {
            self.name.partial_cmp(other.name())
        }
    }
}
impl Ord for Class{
    fn cmp(&self, other: &Class) ->Ordering{
        self.partial_cmp(other).unwrap()
    }
}
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default,PartialOrd,Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Feature{
    name: String,
    description: String,
}
impl Feature{
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn description(&self)->&String{
        &self.description
    }
    pub fn new(name: String, description: String)->Feature{
        Feature { name, description}
    }
}
///a background is a name and a description
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Background{
    name: String,
    background_feature: Feature,
}
impl Background{
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn background_feature(&self)->&Feature{
        &self.background_feature
    }
    pub fn set_background_feature(&mut self,feat: Feature){
        self.background_feature=feat;
    }
    pub fn new(name: String)-> Background{
        Background { name, background_feature: Feature::default() }
    }
}
///a dice has a size, and we include the number
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Die{
    size: i64,
    num: i64,
}
impl Die{
    pub fn size(&self)->i64{
        self.size
    }
    pub fn num(&self)->i64{
        self.num
    }
    pub fn new(size: i64,num: i64)->Die{
        Die { size, num }
    }
}
impl fmt::Display for Die{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}d{}",self.num,self.size)
    }
}
///an attack bouns can be a regular bonus or DC
#[derive(Debug, Eq, PartialEq,Clone,PartialOrd,Ord,Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AtkBonus{
    Bonus(i64),
    DC(i64),
}
impl fmt::Display for AtkBonus{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            AtkBonus::Bonus(k)=>{
                if k>&0{
                    write!(f,"+{}",k)
                } else {
                    write!(f,"{}",k)
                }
            },
            AtkBonus::DC(k) =>{
                write!(f,"DC {}",k)
            }
        }
    }
}
impl Default for AtkBonus{
    fn default()->Self{AtkBonus::Bonus(0)}
}
///an attack is a string, AtkBonus, and damage
#[derive(Debug, Eq, PartialEq, Clone,PartialOrd,Ord,Default,Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Attack{
    name: String,
    bonus: AtkBonus,
    damage: String
}
impl Attack{
    /// returns the name of the attack
    pub fn name(&self)->&String{
        &self.name
    }
    /// returns the attack's bonus
    pub fn bonus(&self)->&AtkBonus{
        &self.bonus
    }
    /// returns the bonus as a string
    pub fn bonus_as_string(&self)->String{
        self.bonus.to_string()
    }
    /// returns the damage
    pub fn damage(&self)->&String{
        &self.damage
    }
    pub fn new(name: String,bonus: AtkBonus,damage: String)->Attack{
        Attack { name , bonus, damage }
    }
    ///adds damage to the attack
    /// #Example
    /// ```
    /// use dicecloud_sheet_printer::holding_structs::{Attack, AtkBonus};
    /// let mut atk1 = Attack::new("test".to_string(),AtkBonus::Bonus(0),"1d8+3 [fire]".to_string());
    /// let mut atk2 = Attack::new("test".to_string(),AtkBonus::Bonus(0),String::new());
    /// atk1.add_dmg("4 [pir]".to_string());
    /// atk2.add_dmg("4 [pir]".to_string());
    ///
    /// assert_eq!(atk1.damage(),"1d8+3 [fire] 4 [pir]");
    /// assert_eq!(atk2.damage(),"4 [pir]");
    /// ```
    pub fn add_dmg(&mut self, dmg: String){
        if !self.damage.is_empty(){
            let _= write!(self.damage," {}",dmg);
        }
        else{
            self.damage=dmg;
        }
    }
}
///an item has a quantity and a name
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Item{
    quantity: i64,
    name: String,
    plural_name: String,
    requires_attunement: bool,
}
impl Item{
    pub fn quantity(&self)->i64{
        self.quantity
    }
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn plural_name(&self)->&String{
        &self.plural_name
    }
    pub fn requires_attunement(&self)->bool{
        self.requires_attunement
    }
    pub fn new(quantity: i64,name: String,plural_name: String)->Item{
        Item { quantity, name,plural_name,requires_attunement: false}
    }
    pub fn needs_attuned(&mut self){
        self.requires_attunement=true;
    }
}
impl PartialOrd for Item{
    fn partial_cmp(&self,other: &Item)->Option<Ordering>{
        if &self.name != other.name(){
            return self.name.partial_cmp(other.name());
        }
        self.quantity.partial_cmp(&other.quantity())
    }
}
impl Ord for Item{
    fn cmp(&self,other: &Item)->Ordering{
        self.partial_cmp(other).unwrap()
    }
}
impl fmt::Display for Item{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let atn = if self.requires_attunement{"❂ "} else {""};
        let nme = if self.quantity==1 {&self.name} else {&self.plural_name};
        write!(f,"{}{} {}",atn,self.quantity,nme)
    }
}
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default,PartialOrd,Ord,Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SpellPrep{
    AlwaysPrepared,
    Prepared,
    #[default]
    NotPrepared,
}
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default,PartialOrd,Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Spell{
    name: String,
    level: i64,
    casting_time: ActionType,
    duration: String,
    school: String,
    range: String,
    vscr: (bool,bool,bool,bool),
    material: String,
    prepd: SpellPrep
}
impl Spell{
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn level(&self)->i64{
        self.level
    }
    pub fn casting_time(&self)->&ActionType{
        &self.casting_time
    }
    pub fn duration(&self)->&String{
        &self.duration
    }
    pub fn school(&self)->&String{
        &self.school
    }
    pub fn range(&self)->&String{
        &self.range
    }
    pub fn vscr(&self)->(bool,bool,bool,bool){
        self.vscr
    }
    pub fn vscr_to_string(&self)->String{
        let v = if self.vscr.0{"v"} else {""};
        let s = if self.vscr.1{"s"} else {""};
        let c = if self.vscr.2{"c"} else {""};
        let r = if self.vscr.3{"r"} else {""};
        format!("{}{}{}{}",v,s,c,r)
    }
    pub fn material(&self)->&String{
        &self.material
    }
    pub fn prepd(&self)->SpellPrep{
        self.prepd
    }
    pub fn new(name: String, level: i64, casting_time: ActionType, duration: String, school: String, range: String, vscr: (bool,bool,bool,bool),material: String)->Spell{
        Spell{name,level,casting_time,duration,school,range,vscr,material,prepd:SpellPrep::NotPrepared}
    }
    pub fn prepare(&mut self){
        self.prepd = SpellPrep::Prepared;
    }
    pub fn always_prepare(&mut self){
        self.prepd = SpellPrep::AlwaysPrepared;
    }
    pub fn unprepare(&mut self){
        self.prepd = SpellPrep::NotPrepared;
    }
}
///We store all spells of the same level in the same SpellLevel struct
#[derive(Debug, Eq, PartialEq,Clone,Hash,Default,PartialOrd,Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpellLevel{
    level: i64,
    spells: Vec<Spell>
}
impl SpellLevel{
    pub fn lvl(&self)->i64{
        self.level
    }
    pub fn spells(&self)->&Vec<Spell>{
        &self.spells
    }
    pub fn new(level: i64, spells: Vec<Spell>)->SpellLevel{
        SpellLevel { level, spells }
    }
    pub fn add_spell(&mut self,spell: Spell){
        self.spells.push(spell);
    }
}
///a spell list has spells of several levels, but with a casting class, ability, save dc, and attack bonus
#[derive(Debug, Eq, PartialEq,Clone,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpellList{
    pub levels: HashMap<i64,SpellLevel>,
    pub name: String,
    pub save_dc: i64,
    pub atk_bonus: i64,
    pub max_prepared: i64
}
impl SpellList{
    pub fn new(levels: HashMap<i64,SpellLevel>,name: String,save_dc: i64, atk_bonus: i64, max_prepared: i64)->SpellList{
        SpellList{levels, name, save_dc, atk_bonus,max_prepared}
    }
    pub fn max_lvl(&self)->i64{
        self.levels.iter().fold(0,|mx,val| if val.1.lvl()>mx {val.1.lvl()} else {mx})
    }
}
impl PartialOrd for SpellList{
    fn partial_cmp(&self,other: &SpellList)->Option<Ordering>{
        let own_lvl = self.max_lvl();
        let other_lvl = other.max_lvl();
        if self.max_prepared != other.max_prepared{
            other.max_prepared.partial_cmp(&self.max_prepared)
        } else if own_lvl != other_lvl{
            other_lvl.partial_cmp(&own_lvl)
        } else if self.name != other.name {
            self.name.partial_cmp(&other.name)
        } else if self.save_dc != other.save_dc{
            self.save_dc.partial_cmp(&other.save_dc)
        } else {
            self.atk_bonus.partial_cmp(&other.atk_bonus)
        }
    }
}
impl Ord for SpellList{
    fn cmp(&self,other: &SpellList)->Ordering{
        self.partial_cmp(other).unwrap()
    }
}
///a damage multiplier has Immunity, Resistence, Vulnerability, each with a string damage type
#[derive(Debug, Eq, PartialEq,Clone,Hash,PartialOrd,Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DamageMult{
    Immune(String),
    Resist(String),
    Vuln(String),
}
impl fmt::Display for DamageMult{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (typ, str) = match self{
            DamageMult::Immune(s)=> (s, "immunity"),
            DamageMult::Resist(s)=> (s, "resistance"),
            DamageMult::Vuln(s)=> (s, "vulnerability")
        };
        write!(f,"{} {}",typ,str)
    }
}
#[derive(Debug, Eq, PartialEq,Clone,Hash,PartialOrd,Ord,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ActionType{
    Free,
    Reaction,
    Bonus,
    #[default]
    Action,
    Long(String)
}
impl fmt::Display for ActionType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self{
            ActionType::Free => "fr.".to_string(),
            ActionType::Reaction => "rxn".to_string(),
            ActionType::Bonus => "bns".to_string(),
            ActionType::Action =>"a".to_string(),
            ActionType::Long(time) =>time.to_string(),
        };
        write!(f, "{}", &out)
    }
}
#[derive(Debug, Eq, PartialEq,Clone,Hash,PartialOrd,Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Action{
    name: String,
    typ: ActionType,
    uses: i64 //-1=infty
}
impl Action{
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn uses(&self)->i64{
        self.uses
    }
    pub fn typ(&self)->&ActionType{
        &self.typ
    }
    pub fn new(name: String,uses: i64,typ: ActionType)->Action{
        Action{name, uses,typ}
    }
}
impl fmt::Display for Action{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nl = format!("{}",self.uses).len();
        let resources = if self.uses==-1{
            String::new()
        } else {
            let blank = String::from_utf8(vec![b'_'; nl]).expect("never fails");
            format!("({}/{})",blank,self.uses)
        };
        write!(f,"({}) {}{}",self.typ,self.name,resources)
    }
}
impl Default for Action{
    fn default()->Action{
        Action{name:String::default(),typ: ActionType::default(), uses:-1}
    }
}
#[derive(Debug, Eq, PartialEq,Clone,Hash,PartialOrd,Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Resource {
    name: String,
    total: i64,
}
impl Resource{
    pub fn name(&self)->&String{
        &self.name
    }
    pub fn total(&self)->i64{
        self.total
    }
    pub fn new(name: String, total: i64)->Resource{
        Resource{name, total}
    }
}
impl fmt::Display for Resource{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result{
        let nl = format!("{}",self.total).len();
        let resources = if self.total==-1{
            String::new()
        } else {
            let blank = String::from_utf8(vec![b'_'; nl]).expect("never fails");
            format!("({}/{})",blank,self.total)
        };
        write!(f,"{} {}",self.name,resources)
    }
}
///a struct for parsing the character into
#[derive(Debug, Eq, PartialEq,Clone,Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Character{
    pub name: String,
    pub classes: Vec<Class>,
    pub background: Background,
    pub damage_mults: Vec<DamageMult>,
    pub race: String,
    pub alignment: String,
    pub xp: i64,
    pub ability_scores: Vec<AbilityScore>,
    pub prof_bonus: i64,
    pub saving_throws: Vec<Skill>,
    pub skills: Vec<Skill>,
    pub ac: i64,
    pub passive_bonus: i64,
    pub initiative: i64,
    pub speed: i64,
    pub hit_points: i64,
    pub hit_dice: Vec<Die>,
    pub attacks: Vec<Attack>,
    pub actions: Vec<Action>,
    pub resources: Vec<Resource>,
    pub equipment: Vec<Item>,
    pub traits: (String, String, String, String),//Personality, Ideals, Bonds, Flaws
    pub features: Vec<String>,
    pub other_profs: (Vec<String>,Vec<String>,Vec<String>,Vec<String>),//armor,weapon,language, tool
    pub coins: (i64,i64,i64,i64,i64),//cp,sp,ep,gp,pp
    pub spell_lists: Vec<SpellList>,
    pub spell_slots: [i64;9],//1st,2nd,...9th
    pub char_img: Option<String>,
}
impl Character{
    pub fn from_flat_char(flat_char: FlatCharacter)->Result<Character,String>{
        let info = flat_char.creatures.into_iter().nth(0).ok_or("missing character info".to_string())?;
        let name = info.name;
        let alignment = info.alignment;
        let xp = info.denormalized_stats.xp as i64;
        let char_img = match info.avatar_picture{
            Some(s) => Some(s),
            None => info.picture
        };
        let mut ability_scores: Vec<AbilityScore> = vec![];
        let mut skills: Vec<Skill> = vec![];
        let mut saving_throws: Vec<Skill> = vec![];
        let mut initiative: i64=0;
        let mut prof_bonus: i64=0;
        let mut damage_mults: Vec<DamageMult> = vec![];
        let mut passive_bns: i64 =0;
        let mut speed: i64=0;
        let mut hit_points: i64=0;
        let mut ac: i64=0;
        let mut traits = (String::new(),String::new(),String::new(),String::new());
        let mut attacks_dict: HashMap<String,Attack> =HashMap::new();
        let mut attacks: Vec<Attack>=vec![];
        let mut actions: Vec<Action>=vec![];
        let mut classes: Vec<Class> = vec![];
        let mut features: Vec<String> = vec![];
        let mut starting_class = String::new();
        let mut resources: Vec<Resource> = vec![];
        let mut equipment: Vec<Item> = vec![];
        let mut hit_dice: Vec<Die> = vec![];
        let mut background: Background=Background::default();
        let mut race: String = String::new();
        let mut coins = (0,0,0,0,0);
        let mut other_profs: (Vec<String>,Vec<String>,Vec<String>,Vec<String>) = (vec![],vec![],vec![],vec![]);
        let mut spell_ls_dict: HashMap<String,SpellList> =HashMap::new();
        let mut spell_slots: [i64;9] = [0;9];
        let mut props = flat_char.creature_properties;
        props.sort_by(|a,b| a.partial_cmp(b).unwrap());
        for prop in props.into_iter().filter(|p| !p.removed){
            match prop.prop_type{
                PropType::Attribute { name,variable_name: _,attribute_type,
                    base_value: _, description: _, damage: _,decimal: _, ignore_lower_limit: _,
                    ignore_upper_limit: _,hide_when_value_zero:_, hide_when_total_zero:_, reset:_,
                    total, value, effects:_, hide:_, overridden }=>{
                        if overridden{
                            continue;
                        }
                        match attribute_type{
                            AttributeType::Ability { modifier:_, proficiency:_ }=>{
                                let score = value.as_i64()
                                    .ok_or(attribute_type_missmatch(&name, &value))?;
                                ability_scores.push(AbilityScore { score, name });
                            }
                            AttributeType::HitDice { hit_dice_size, constitution_mod:_ }=>{
                                let size: i64=match hit_dice_size.split('d').collect::<Vec<_>>()[1].parse(){
                                    Ok(k)=>k,
                                    Err(e)=>return Err(e.to_string())
                                };
                                let num = value.as_i64().ok_or(format!("Number of hitdice, {}, has incorrect type",value))?;
                                hit_dice.push(Die { size, num});
                            }
                            AttributeType::SpellSlot { spell_slot_level }=>{
                                if !prop.inactive{
                                    let lvl = spell_slot_level
                                        .ok_or(missing("Spell Slot level",&name))?.value
                                        .as_i64().ok_or("Attribute {} with value {} has the wrong type")?
                                        as usize;
                                    let num = value.as_i64()
                                        .ok_or(attribute_type_missmatch(&name,&value))?;
                                    spell_slots[lvl-1]+=num;

                                }
                            }
                            AttributeType::Resource {  }=>{
                                resources.push(Resource { name, total: total.as_i64().unwrap_or(0) });
                            }
                            AttributeType::HealthBar { health_bar_color_mid:_,
                                health_bar_color_low:_, health_bar_no_damage:_, health_bar_no_healing:_,
                                health_bar_no_damage_overflow:_, health_bar_no_healing_overflow:_,
                                health_bar_damage_order:_, health_bar_healing_order:_ }=>{
                                if name=="Hit Points"{
                                    hit_points=value.as_i64().ok_or(type_missmatch("Hit Points","",&value))?;
                                }
                            }
                            _=>{if name=="Proficiency Bonus"{
                                prof_bonus=value.as_i64().ok_or(type_missmatch("Proficiency Bonus","",&value))?;
                            }else if name=="Speed"{
                                speed=total.as_i64().ok_or(type_missmatch("Speed","",&value))?;
                            }else if name=="Armor Class"{
                                ac = total.as_i64().ok_or(type_missmatch("AC","",&value))?;
                            }}
                        }
                    },
                PropType::Skill { name, variable_name:_, ability:_,
                    skill_type, base_proficiency:_, base_value:_,
                    description:_, value, ability_mod:_, advantage:_,
                    passive_bonus, proficiency, conditional_benifits:_, fail:_,
                    hide:_, overridden:_, effects:_ }=>{
                        if &name == "Initiative"{
                            initiative=value;
                        } else if skill_type == "save"{
                            let prof = if proficiency==0.49 || proficiency==0.5{
                                Proficiency::Half
                            } else if proficiency==1.0{
                                Proficiency::Profficient
                            } else if proficiency==2.0{
                                Proficiency::Expert
                            } else {
                                Proficiency::None
                            };
                            saving_throws.push(Skill::new(name,value,prof));
                        } else if skill_type == "skill"{
                            let prof = if proficiency==0.49 || proficiency==0.5{
                                Proficiency::Half
                            } else if proficiency==1.0{
                                Proficiency::Profficient
                            } else if proficiency==2.0{
                                Proficiency::Expert
                            } else {
                                Proficiency::None
                            };
                            if name == "Perception"{
                                passive_bns=passive_bonus;
                            }
                            skills.push(Skill::new(name, value, prof));
                        } else if skill_type == "armor"{
                            other_profs.0.push(name);
                        } else if skill_type=="weapon"{
                            other_profs.1.push(name);
                        } else if skill_type=="language"{
                            other_profs.2.push(name);
                        } else if skill_type=="tool"{
                            other_profs.3.push(name);
                        }
                    },
                PropType::Feature { name, sumary:_,
                    description }=>{
                        if prop.tags.iter().any(|t| t.contains("background")){
                            let description = match description{
                                Some(c)=>c.text,
                                None=> String::new()
                            };
                            background.set_background_feature(Feature::new(name,
                                description));
                        } else {
                            features.push(name);
                        }
                },
                PropType::SpellList { name, max_prepared,
                    dc, attack_roll_bonus,
                    description:_, ability:_, ability_mod:_ }=>{
                        let max_prepared = match max_prepared{
                            Some(c) => {
                                c.value.as_i64()
                                    .ok_or(type_missmatch("SpellList max prepared",
                                        &name, &c.value))?},
                            None=>0
                        };
                        let dc = match dc{
                            Some(c)=>{
                                c.value.as_i64().ok_or(type_missmatch("SpellList dc", &name, &c.value))?
                            },
                            None => 0
                        };
                        let attack_bonus = match attack_roll_bonus{
                            Some(c)=>{
                                c.value.as_i64().ok_or(type_missmatch("SpellList attack bonus", &name, &c.value))?
                            }
                            None=>0
                        };
                        spell_ls_dict.entry(prop.id).or_insert(
                            SpellList::new(HashMap::new(),name.to_string(),dc,attack_bonus,max_prepared));
                },
                PropType::Spell { name, always_prepared, prepared,
                    cast_without_spell_slots:_, has_attack_roll:_, casting_time,
                    range, duration, verbal, somatic,
                    concentration, material, ritual, level, school,
                    summary:_, description:_, action_type:_,
                    variable_name:_, target:_, attack_roll:_,
                    uses:_, uses_used:_, reset:_, silent:_,
                    resources:_, insufficient_resources:_, uses_left:_, overridden:_,
                    deactivated_by_self:_ }=>{
                    if prop.deactivated_by_ancestor == Some(true){
                        continue;
                    }
                    let ancestors = prop.ancestors;
                    let mut spell_list_id = String::new();
                    for anc in ancestors.iter().rev(){
                        if spell_ls_dict.contains_key(&anc.id){
                            spell_list_id=anc.id.clone();
                            break;
                        }
                    }
                    let vscr = (verbal,somatic,concentration,ritual);
                    let material = material.unwrap_or_default();
                    let duration = duration.unwrap_or_default();
                    let range = range.unwrap_or_default();
                    let casting_time = casting_time.unwrap_or("long".to_string());
                    let casting_time = if &casting_time=="action"{
                        ActionType::Action
                    } else if &casting_time=="bonus"{
                        ActionType::Bonus
                    } else if casting_time.contains("reaction"){
                        ActionType::Reaction
                    } else if &casting_time=="free"{
                        ActionType::Free
                    } else {
                        ActionType::Long(casting_time.replace("round","rnd")
                            .replace("minute","min").replace("hour","hr"))
                    };
                    let duration = duration.to_lowercase().replace("up to ","").replace("round","rnd")
                        .replace("minute","min").replace("hour","hr");
                    let range = range.replace("feet","ft").replace("miles","mi")
                        .replace("mile","mi").replace("slotLevel","sl")
                        .replace("foot","ft").replace("radius","rad")
                        .replace(" * (1 + spellSniper)","");
                    let mut spl = Spell::new(name,level,casting_time,duration, school, range, vscr, material);
                    if always_prepared{
                        spl.always_prepare();
                    } else if prepared{
                        spl.prepare();
                    }
                    let _=spell_ls_dict.entry(spell_list_id.to_string()).and_modify(|ls|{
                        ls.levels.entry(level).and_modify(|splvl| {splvl.add_spell(spl.clone());});}
                    );
                }
                PropType::DamageMultiplier { name:_, damage_types, value,
                    exclude_tags:_, include_tags:_ }=>{
                        if value == 0.0{
                            for typ in damage_types{
                                damage_mults.push(DamageMult::Immune(typ));
                            }
                        } else if value == 0.5{
                            for typ in damage_types{
                                damage_mults.push(DamageMult::Resist(typ));
                            }
                        } else if value == 2.0{
                            for typ in damage_types{
                                damage_mults.push(DamageMult::Vuln(typ));
                            }
                        }
                },
                PropType::Note { name, summary,
                    description:_ }=>{
                    if name == "Flaws"{
                        traits.3= match summary{
                            Some(t)=> t.text,
                            None=>String::new(),
                        };
                    } else if name == "Ideals"{
                        traits.1 = match summary{
                            Some(t)=>t.text,
                            None=>String::new()
                        };
                    } else if name == "Personality Traits"{
                        traits.0 = match summary{
                            Some(t)=>t.text,
                            None=>String::new()
                        };
                    }else if name == "Bonds"{
                        traits.2 = match summary{
                            Some(t)=>t.text,
                            None=>String::new()
                        };
                    }
                }
                PropType::Action { name, summary:_,
                    description:_, action_type,
                    variable_name:_, target:_, attack_roll,
                    uses, uses_used:_, reset:_, silent:_, resources:_,
                    insufficient_resources:_, uses_left:_, overridden:_ }=>{
                    
                    if action_type=="attack" && !prop.inactive{
                        let bonus = match attack_roll{
                            Some(c)=>{
                                c.value.as_i64().ok_or(type_missmatch("Attack roll",&name,&c.value))?
                            },
                            None=>0
                        };
                        attacks_dict.insert(prop.id, Attack { name,bonus: AtkBonus::Bonus(bonus), damage:String::new()});
                    } else {
                        let uses = match uses{
                            Some(val)=>{
                                val.value.as_i64().ok_or(type_missmatch("Action uses", &name,&val.value))?
                            },
                            None =>-1
                        };
                        let typ = if action_type=="free"{
                            ActionType::Free
                        } else if action_type=="bonus"{
                            ActionType::Bonus
                        } else if action_type=="reaction"{
                            ActionType::Reaction
                        } else if action_type=="action"{
                            ActionType::Action
                        } else if action_type=="event"{
                            continue
                        } else if action_type=="long"{
                            ActionType::Long("lng".to_string())
                        } else {
                            ActionType::default()
                        };
                        actions.push(Action{name,uses,typ});
                    }
                },
                PropType::Damage { amount, target:_, damage_type, silent:_ }=>{
                    let par_id = prop.parent.id;
                    let (dmg_die, dmg_bonus)= match amount{
                        Some(c)=>(c.value.to_string(),
                            c.effects.into_iter().fold(0,|b,e|
                            e.amount.value.as_i64().unwrap_or(0)+b
                            )),
                        None=>("0d0".to_string(),0)
                    };
                    let dmg_string = format!("{}{}{}[{}]",dmg_die,if dmg_bonus>=0 {"+"} else {""},
                        dmg_bonus,damage_type_abreviator(damage_type));
                    attacks_dict.entry(par_id).and_modify(|a| 
                        a.add_dmg(dmg_string));
                },
                PropType::Class { name, description:_,
                    variable_name:_, slot_tags:_, extra_tags:_,
                    slot_condition:_, level, missing_levels:_ }=>{
                        classes.push(Class::new(name,level));
                    },
                PropType::Item { name, plural, description:_, 
                    quantity, weight:_, value:_, requires_attunement, attuned:_,
                    show_increment:_, equipped:_ }=>{

                    if name.to_lowercase().contains("piece"){
                        let coin_type = name.to_lowercase();
                        if coin_type.contains("platinum"){
                            coins.4 = quantity;
                        } else if coin_type.contains("gold"){
                            coins.3 = quantity;
                        } else if coin_type.contains("electrum"){
                            coins.2 = quantity;
                        }else if coin_type.contains("silver"){
                            coins.1 = quantity;
                        } else if coin_type.contains("copper"){
                            coins.0 = quantity;
                        }
                    } else {
                        let mut itm = Item::new(quantity,name,plural);
                        if requires_attunement{
                            itm.needs_attuned();
                        }
                        equipment.push(itm);
                    }
                },
                PropType::SlotFiller { name, picture:_, description:_,
                    slot_filler_type:_, slot_quantity_filled:_, slot_filler_condition:_ }=>{
                    if prop.tags.contains(&("background".to_string())){
                        background=Background::new(name);
                    }
                },
                PropType::Constant { name:_, variable_name,
                    calculation, errors:_ }=>{
                    if variable_name==Some("race".to_string()){
                        if race == String::new(){
                            race = calculation.unwrap_or_default().replace('"',"");
                        }
                    } else if variable_name==Some("subRace".to_string()){
                        race = calculation.unwrap_or_default().replace('"',"");
                    } else if variable_name==Some("startingClass".to_string()){
                        starting_class = calculation.unwrap_or_default().replace('"',"");
                    }
                },
                _=>{}
            }
        }
        for class in classes.iter_mut(){
            if class.name().to_lowercase() == starting_class.replace('\"',"").replace('\'',"").to_lowercase(){
                class.start_class=true;
                break;
            }
        }
        for pair in attacks_dict.into_iter(){
            if !pair.1.name().is_empty(){
                attacks.push(pair.1);
            }
        }
        let mut spell_lists: Vec<SpellList>= vec![];
        for pair in spell_ls_dict.into_iter(){
            if !pair.1.name.is_empty(){
                spell_lists.push(pair.1);
            }
        }
        Ok(Character { name, classes, background, damage_mults, race, alignment, xp, ability_scores,
            prof_bonus, saving_throws, skills, ac, passive_bonus: passive_bns, initiative, speed,hit_points, hit_dice,
            attacks, actions, resources, equipment, traits, features, other_profs, coins, spell_lists,
            spell_slots, char_img})
    }
    
    pub fn from_str(char_json: &str)->Result<Character,String>{
        let flat_char: FlatCharacter=match serde_json::from_str(char_json){
            Ok(c)=>c,
            Err(e)=>return Err(e.to_string())
        };
        Character::from_flat_char(flat_char)
    }
}
pub fn missing(thing: &str, name: &str)->String{
    format!("missing {} for {}",thing,name)
}
pub fn attribute_type_missmatch(name: &str,val: &PropVal)->String{
    format!("Attribute {} with value {} has the wrong type",name,val)
}
pub fn type_missmatch(typ: &str, name: &str, val: &PropVal)->String{
    format!("{} {} with value {} has the wrong type",typ, name,val)
}
fn damage_type_abreviator(typ: String)->String{
    if typ.len()<5{
        return typ;
    }else if &typ == "piercing"{
        return "pir.".to_string();
    }
    let mut typ_bits=typ.into_bytes();
    typ_bits.truncate(3);
    String::from_utf8(typ_bits).expect("should never happen by design")+"."
}