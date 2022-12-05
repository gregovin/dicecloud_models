# FlatProp
represents a property in the JSON. Properties have(or can have) varying components

All properties have
- _id: String,
- type: String,
- tags: Array of String,  
- order: integer(unsigned),  
- parent: [Identifier](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#identifier),  
- ancestors: Array of [Identifier](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#identifier),  
- libraryNodeId*: String,  
- color*: String,  
- icon*: [Icon](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#icon),  
- libraryTags*: array of string,  
- deactivatedByToggle*: bool,  
- deactivatedByAncestor*: bool,  
- inactive*: bool,  
- removed*: bool,  
- removedAt*: String,  
- removedWith*: String  

The following properties have additional fields as follows
## Action
- actionType: String
- target: String
- resources: [Resource](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#resource)
- name: String
- summary*: [CalcuatedText](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#calculatedtext)
- description*: [CalcuatedText](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#calculatedtext) 
- variableName*: String,
- attackRoll*: ExtendedCalc

## Attribute damage(adjustment)
- target: String
- operation: String
- stat: String
- amount: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation)

## Attribute
- name: String
- variableName: String, 
- attributeType: String
- description*: [CalcuatedText](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#calculatedtext),
- baseValue: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation)
- total: PropVal
- value: PropVal,
- damage*: PropVal
- effects: Array of [Effect](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#effect),
- reset*: String,
- ignoreLowerLimit*: bool,
- ignoreUpperLimit*: bool,
- hideWhenValueZero*: bool,
- hideWhenTotalZero*: bool

The following attribute types have additional properties
### ability
- modifier: integer,

### Health Bar
- healthBarColorMid*: String,
- healthBarColorLow*: String,
- healthBarDamageOrder*: integer(probably unsigned),
- healthBarHealingOrder*: integer(probably unsigned)

### Hit Dice
- hitDiceSize: String,
- constitutionMod: integer

### Spellslot
- spell_slot_level*: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation)

## Buff
- target: String
- name: String
- description: [CalcuatedText](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#calculatedtext),
- skipCrystalization*: bool,
- hideRemoveButton*: bool,
- silent*: bool

## Buff Remover
- target: String
- remove_all*: bool,
- name: String,
- targetParentBuff*: bool,
- silent: bool

## Branch
All branches have
- branchType: String
only If and Index branches have any extra properties, both with
- condition: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation)

## Class
- name: String,
- variableName: String,
- extraTags: array of [ExtraTag](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#extratag),
- slotCondition*: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation),
- level: i64

## Class Level
- name: String,
- level: i64,
- variableName: String

## Constant
- name: String,
- variableName: String,
- calculation: String,
- errors: array of [ParseError](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parseerror)

## Container
- name: String,
- carried*: bool,
- value*: decimal(float),
- weight*: decimal,
- carried_value*: decimal,
- carried_weight*: decimal,
contents_value*: decimal,
contents_weight*: decimal

## Damage
- target: String,
- damageType: String,
- amount: ExtendedCalc,
- silent*: bool

## Damage Multiplier
- name: String,
- damageTypes: Array of String,
- value: decimal,
- excludeTags*: Array of String,
- include_tags*: Array of String

## Effect
(note effect properties and [Effect](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#effect) have differing structures)
- name: String
- operation: String
- stats: Array of String
- amount: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation),
- targetByTags*: bool,
- targetTags*: Array of String,
- extraTags: Array of [ExtraTag](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#extratag)
- targetField*: String

## Feautre
- name: String
- sumary*: [CalcuatedText](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#calculatedtext),
- description*: [CalcuatedText](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#calculatedtext)

## Folder
- name: String,
- groupStats*: bool,
- hideStatsGroup*: bool,
- location*: String,
- tab*: String

## Item
- name: String,
- plural: String,
- quantity: integer(unsigned),
- equipped: bool,
- value*: decimal,
- weight*: decimal,
- show_increment*: bool,
- description*: [CalcuatedText](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#calculatedtext),
- requires_attunement*: bool,
- attuned*: bool

## Note
- name: String
- summary: [CalcuatedText](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#calculatedtext),
- description: [CalcuatedText](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#calculatedtext)

## Point Buy
- name: String,
- min: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation),
- max: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation),
- values: Array of PointBuyRow,
- cost: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation)
- total: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation),
- pointsLeft: integer,
- spent: integer,
- ignored*: bool

### where PointBuyRow is
- _id: String,
- name: String,
- variableName: String,
- value: integer,
- spent: integer

## Proficiency
- name: String,
- stats: Array of String,
- value: decimal

## Slot (aka propertySlot in the api)
- name: String,
- description*: [CalcuatedText](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#calculatedtext),
- slotTags: Array of String,
- slotType*: String,
- quantityExpected: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation),
- hideWhenFull: bool,
- spaceLeft: integer,
- totalFilled: integer,
- extraTags: Array of [ExtraTag](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#extratag),
- unique: String,
- ignored*: bool

## Roll
- name: String, 
- variableName: String,
- roll: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation)

## Saving Throw
- name*: String,
- target: String,
- stat: String,
- dc: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation),
- silent*: bool

## Skill
- name: String,
- skillType: String,
- variableName: String,
- ability: String,
- description*: [CalcuatedText](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#calculatedtext),
- abilityMod: integer,
- proficiency: decimal,
- value: integer,
- baseValue*: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation),
- baseProficiency*: decimal

## Slotfiller
- name: String, 
- slotQuantityFilled: integer,
- description: String, 
- picture*: String,
- slotFillerType*: String,
- slotFillerCondition*: String

## Spell List
- name: String, 
- maxPrepared*: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation),
- dc*: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation),
- attack_roll_bonus*: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation),
- description*: [CalcuatedText](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#calculatedtext),
- ability: String,
- abilityMod*: integer

## Spell
- name: String,
- actionType: String,
- target: String,
- resources: [Resource](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#resource),
- castingTime: String,
- duration: String,
- level: integer,
- school: String,
- range: String, 
- verbal: bool,
- somatic: bool,
- material*: String,
- concentration*: bool,
- ritual*: bool,
- description*: [CalcuatedText](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#calculatedtext),
- summary: [CalcuatedText](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#calculatedtext),
- prepared: bool,
- alwaysPrepared*: bool,
- deactivatedBySelf*: bool

## Toggle
- name: String,
- condition*: [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation),
- show_ui*: bool,
- disabled*: bool,
- enabled*: bool,
- deactivatedBySelf*: bool