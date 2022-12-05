## Identifier
- id: String,  
- collection: String  

## Icon
- name: String,
- shape: String

## Resource
- items_consumed: Array of [ConsumedItem](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#consumeditem),
- attributes_consumed: Array of [ConsumedResource](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#consumedresource)

## ConsumedItem
- _id: String,
- tag: String,
- quantity: Calculation,

## ConsumedResource
- id: String,
- variable_name: String,
- quantity: Calculation,
- available: i64,
- stat_name: String,

## CalculatedText
- text: String,
- value: String,
- hash: integer,
- inlineCalculations: Array of Calculation

## ExtraTag
- _id: String,
- operation: String,
- tags: Array of String

## Effect
- _id: String,
- name: String,
- operation: String,
- amount:[ValWrap](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#valwrap),
- type*: String