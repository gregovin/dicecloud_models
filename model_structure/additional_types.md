## Identifier
- id: String,  
- collection: String  

## Icon
- name: String,
- shape: String

## Resource
- items_consumed: Array of ConsumedItem,
- attributes_consumed: Array of ConsumedResource

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