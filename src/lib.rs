pub mod data_models;

#[cfg(test)]
#[cfg(feature="serde_json")]
mod tests {
    use crate::data_models::generic_model::*;
    #[test]
    fn death_save_info() {
        let sve=DeathSaveInfo{pass:2,fail:1,can_death_save:true,stable:false};
        let serialized = serde_json::to_string(&sve).unwrap();
        assert_eq!(serialized,"{\"pass\":2,\"fail\":1,\"canDeathSave\":true,\"stable\":false}");

        let tst = "{\"pass\":0,\"fail\":0,\"canDeathSave\":false,\"stable\":true}";
        let deser: DeathSaveInfo = serde_json::from_str(tst).unwrap();
        assert_eq!(deser, DeathSaveInfo{pass: 0, fail: 0, can_death_save:false,stable:true});
    }
    #[test]
    fn denormalized_stats(){
        let sts=DenormalizedStats{xp:10,milestone_levels:2};
        let serialized = serde_json::to_string(&sts).unwrap();
        assert_eq!(serialized,"{\"xp\":10,\"milestoneLevels\":2}");

        let tst = "{\"xp\": 0,\"milestoneLevels\": 1}";
        let deser: DenormalizedStats = serde_json::from_str(tst).unwrap();
        assert_eq!(deser,DenormalizedStats{xp: 0,milestone_levels:1});
    }
    #[test]
    fn settings(){
        let tst = "{\"showTreeTab\": true,\"hideUnusedStats\": true}";
        let deser: Settings = serde_json::from_str(tst).unwrap();
        assert_eq!(deser,Settings{show_tree_tab:true,hide_rest_buttons:false,hide_unused_stats:true,hide_spells_tab:false});
    }
    #[test]
    fn creature_info(){
        let (id,own,lib1,lib2)=("this".to_string(),"is".to_string(),"a".to_string(),"test".to_string());
        let tst =format!("{{\"_id\": \"{}\",
            \"owner\": \"{}\",
            \"name\": \"sorcadin\",
            \"gender\": \"male\",
            \"alignment\": \"neutral good\",
            \"allowedLibraries\": [\"{}\",\"{}\"],
            \"allowedLibraryCollections\": [],
            \"deathSave\": {{
                \"pass\": 0,
                \"fail\": 0,
                \"canDeathSave\": true,
                \"stable\": false
            }},
            \"denormalizedStats\": {{
                \"xp\": 0,
                \"milestoneLevels\": 1
            }},
            \"type\": \"pc\",
            \"damageMultipliers\": {{}},
            \"variables\": {{}},
            \"settings\": {{
                \"showTreeTab\": true,
                \"hideRestButtons\": true,
                \"hideUnusedStats\": true,
                \"hideSpellsTab\": true
            }},
            \"readers\": [],
            \"writers\": [],
            \"public\": true
        }}",id,own,&lib1,&lib2);
        let deser: CreatureInfo=serde_json::from_str(&tst).unwrap();
        let death_save = DeathSaveInfo{pass: 0,fail: 0, can_death_save: true, stable:false};
        let denormalized_stats = DenormalizedStats{xp: 0,milestone_levels:1};
        let settings = Settings{show_tree_tab:true,hide_rest_buttons:true,hide_unused_stats:true,hide_spells_tab:true};
        let char_info = CreatureInfo{id,owner: own,name: "sorcadin".to_string(),gender: "male".to_string(),
            alignment:"neutral good".to_string(),allowed_libraries:vec![lib1,lib2],allowed_library_collections:vec![],
            death_save, denormalized_stats,typ: "pc".to_string(),settings,readers: vec![],writers: vec![],public:true,
            picture: None,avatar_picture: None};
        assert_eq!(deser,char_info);
    }
    #[test]
    fn prop_val(){
        let tst1="1";
        let tst2="\"test\"";
        let tst3="true";
        let tst4="1.5";
        let deser1: PropVal = serde_json::from_str(tst1).unwrap();
        let deser2: PropVal = serde_json::from_str(tst2).unwrap();
        let deser3: PropVal = serde_json::from_str(tst3).unwrap();
        let deser4: PropVal = serde_json::from_str(tst4).unwrap();
        assert_eq!(deser1,PropVal::Number(1));
        assert_eq!(deser2,PropVal::Str("test".to_string()));
        assert_eq!(deser3,PropVal::Boolean(true));
        assert_eq!(deser4,PropVal::Fraction(1.5.into()));
    }
    #[test]
    fn val_wrap(){
        let tst="{\"value\":1}";
        let deser: ValWrap = serde_json::from_str(tst).unwrap();
        assert_eq!(deser,ValWrap{value: PropVal::Number(1)});
    }
    #[test]
    fn parse_node(){
        let tst = "{
            \"parseType\": \"operator\",
            \"left\": {
                \"parseType\": \"symbol\",
                \"name\": \"carryingCapacity\"
            },
            \"right\": {
                \"parseType\": \"constant\",
                \"valueType\": \"number\",
                \"value\": 2
            },
            \"operator\": \"*\",
            \"fn\": \"multiply\"
        }";
        let deser: ParseNode = serde_json::from_str(tst).unwrap();
        let left = ParseNode::Symbol{name: "carryingCapacity".to_string()};
        let right = ParseNode::Constant{value_type:"number".to_string(), value: PropVal::Number(2)};
        let expected = ParseNode::Operator{left: Box::new(left),right: Box::new(right),operator:"*".to_string(),fun:"multiply".to_string()};
        assert_eq!(deser,expected);
    }
    #[test]
    fn parse_error(){
        let tst = "{\"type\": \"info\",\"message\": \"dne not found, set to 0\"}";
        let deser: ParseError = serde_json::from_str(tst).unwrap();
        assert_eq!(deser,ParseError{typ: "info".to_string(),message: "dne not found, set to 0".to_string()});
    }
    #[test]
    fn calculation(){
        let tst = "{
            \"calculation\": \"dne ? 1 : 0\",\"_key\": \"baseValue\",\"type\": \"_calculation\",\"hash\": 5843567941511658,
            \"parseNode\": {
                \"parseType\": \"if\",\"condition\": {\"parseType\": \"symbol\",\"name\": \"dne\"},
                \"consequent\": {\"parseType\": \"constant\",\"valueType\": \"number\",\"value\": 1},
                \"alternative\": {\"parseType\": \"constant\",\"valueType\": \"number\",\"value\": 0}},
            \"parseError\": null,\"errors\": [{\"type\": \"info\",\"message\": \"dne not found, set to 0\"}],\"value\": 0}";
        let deser: Calculation = serde_json::from_str(tst).unwrap();
        let cond = ParseNode::Symbol{name: "dne".to_string()};
        let cons = ParseNode::Constant{value_type: "number".to_string(),value: PropVal::Number(1)};
        let alt = ParseNode::Constant{value_type: "number".to_string(),value: PropVal::Number(0)};
        let err= ParseError{typ: "info".to_string(),message: "dne not found, set to 0".to_string()};
        let calc = Calculation{calculation: "dne ? 1 : 0".to_string(),key:"baseValue".to_string(),
            typ: "_calculation".to_string(),hash:5843567941511658,parse_node: ParseNode::If{condition: Box::new(cond),
                consequent: Box::new(cons),alternative: Box::new(alt)},errors: vec![err],value: PropVal::Number(0),
                base_value: None, effects: vec![]};
        assert_eq!(deser, calc);
    }
    #[test]
    fn identifier(){
        let tst="{\"collection\": \"creatures\",\"id\": \"cRdjQ9HKMzsBcBTSE\"}";
        let tst2="{\"id\": \"i2nR4kWmBREB8A8iL\",\"collection\": \"creatureProperties\"}";
        let deser: Identifier = serde_json::from_str(tst).unwrap();
        let deser2: Identifier = serde_json::from_str(tst2).unwrap();
        assert_eq!(deser,Identifier{id:"cRdjQ9HKMzsBcBTSE".to_string(),collection:"creatures".to_string()});
        assert_eq!(deser2,Identifier{id:"i2nR4kWmBREB8A8iL".to_string(),collection:"creatureProperties".to_string()});
    }
    #[test]
    fn calculated_text(){
        let tst="{\"text\": \"test\",\"inlineCalculations\": [],\"value\": \"test\",\"hash\": 8030999935138279}";
        let deser: CalculatedText = serde_json::from_str(tst).unwrap();
        assert_eq!(deser,CalculatedText{text: "test".to_string(),value: "test".to_string(),hash: 8030999935138279, inline_calculations: vec![]});
    
    }
    #[test]
    fn effect(){
        let tst = "{\"_id\": \"CtRdeuSYYZCJGD2B2\",\"name\": \"Strength\",\"operation\": \"base\",
            \"amount\": {\"value\": 18},\"type\": \"attribute\"}";
        let deser: Effect = serde_json::from_str(tst).unwrap();
        assert_eq!(deser, Effect{id:"CtRdeuSYYZCJGD2B2".to_string(),name:Some("Strength".to_string()),
            operation: "base".to_string(),amount: ValWrap{value: PropVal::Number(18)},typ: Some("attribute".to_string())});
    }
}
pub mod safe_frac;