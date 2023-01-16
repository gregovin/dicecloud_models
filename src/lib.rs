pub mod data_models;

#[cfg(test)]
#[cfg(feature="serde_json")]
mod tests {
    use crate::data_models::generic_model::*;
    #[test]
    fn prop_val(){
        let tst1="1";
        let tst2="\"test\"";
        let tst3="true";
        let tst4="1.5";
        let tst5 ="null";
        let deser1: PropVal = serde_json::from_str(tst1).unwrap();
        let deser2: PropVal = serde_json::from_str(tst2).unwrap();
        let deser3: PropVal = serde_json::from_str(tst3).unwrap();
        let deser4: PropVal = serde_json::from_str(tst4).unwrap();
        let deser5: PropVal = serde_json::from_str(tst5).unwrap();
        assert_eq!(deser1,PropVal::Number(1));
        assert_eq!(deser2,PropVal::Str("test".to_string()));
        assert_eq!(deser3,PropVal::Boolean(true));
        assert_eq!(deser4,PropVal::Fraction(1.5.into()));
        assert_eq!(deser5,PropVal::None(None));
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
    fn effect(){
        let tst = "{\"_id\": \"CtRdeuSYYZCJGD2B2\",\"name\": \"Strength\",\"operation\": \"base\",
            \"amount\": {\"value\": 18},\"type\": \"attribute\"}";
        let deser: Effect = serde_json::from_str(tst).unwrap();
        assert_eq!(deser, Effect{id:"CtRdeuSYYZCJGD2B2".to_string(),name:Some("Strength".to_string()),
            operation: "base".to_string(),amount: ValWrap{value: PropVal::Number(18)},typ: Some("attribute".to_string())});
    }
}