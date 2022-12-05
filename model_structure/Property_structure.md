# FlatProp represents a property in the JSON. Properties have(or can have) varying components
All properties have
-_id: String,
-type: String
-tags: Vec<String>,
-order: integer(unsigned),
-parent: Identifier,
-ancestors: Array of Identifier,
-libraryNodeId*: String,
-color*: String,
-icon*: Icon,
-libraryTags*: array of string,
-deactivatedByToggle*: bool,
-deactivatedByAncestor*: bool,
-inactive*: bool,
-removed*: bool,
-removedAt*: String,
-removedWith*: String

## Identifier
-id: String,
-collection: String

## Icon
-name: String,
-shape: String

The following properties have additional fields as follows
## Action
-actionType: String
-target: String
-resources: Resource
-name: String
-summary*: CalculatedText
-description*: CalculatedText 
-variableName*: String,
-attackRoll*: ExtendedCalc

## Attribute damage(adjustment)
-target: String
-operation: String
-stat: String
-amount: Type Calculation

## Attribute
-name: String
-variableName: String, 
-attributeType: String
-description*: CalculatedText,
-baseValue: Calculation
-total: PropVal
-value: PropVal,
-damage*: PropVal
-effects: Array of Effect,
-reset*: String,
-ignoreLowerLimit*: bool,
-ignoreUpperLimit*: bool,
-hideWhenValueZero*: bool,
-hideWhenTotalZero*: bool

The following attribute types have additional properties
### ability
-modifier: integer,

### Health Bar
-healthBarColorMid*: String,
-healthBarColorLow*: String,
-healthBarDamageOrder*: integer(probably unsigned),
-healthBarHealingOrder*: integer(probably unsigned)

### Hit Dice
- hitDiceSize: String,
- constitutionMod: integer

### Spellslot
-spell_slot_level*: Calculation

## Buff
-target: String
-name: String
-description: CalculatedText,
-skipCrystalization*: bool,
-hideRemoveButton*: bool,
-silent*: bool

## Buff Remover
-target: String
-remove_all*: bool,
-name: String,
-targetParentBuff*: bool,
-silent: bool

## Branch
All branches have
-branchType: String
only If and Index branches have any extra properties, both with
-condition: Calculation

## Class
-name: String,
-variableName: String,
-extraTags: array of ExtraTag,
-slotCondition*: Calculation,
-level: i64

## Class Level
-name: String,
-level: i64,
-variableName: String

## Constant
-name: String,
-variableName: String,
-calculation: String,
-errors: array of ParseError

## Container
-name: String,
-carried*: bool,
-value*: decimal(float),
-weight*: decimal,
-carried_value*: decimal,
-carried_weight*: decimal,
contents_value*: decimal,
contents_weight*: decimal

## Damage
-target: String,
-damageType: String,
-amount: ExtendedCalc,
-silent*: bool

## Damage Multiplier
-name: String,
-damageTypes: Array of String,
-value: decimal,
-excludeTags*: Array of String,
-include_tags*: Array of String

## Effect(note effect properties and Effect have differing structures)
-name: String
-operation: String
-stats: Array of String
-amount: Calculation,
-targetByTags*: bool,
-targetTags*: Array of String,
-extraTags: Array of ExtraTag
-targetField*: String

## Feautre
-name: String
-sumary*: CalculatedText,
-description*: CalculatedText

## Folder
-name: String,
-groupStats*: bool,
-hideStatsGroup*: bool,
-location*: String,
-tab*: String

## Item
-name: String,
-plural: String,
-quantity: integer(unsigned),
-equipped: bool,
-value*: decimal,
-weight*: decimal,
-show_increment*: bool,
-description*: CalculatedText,
-requires_attunement*: bool,
-attuned*: bool

## Note
-name: String
-summary: CalculatedText,
-description: CalculatedText

## Point Buy
-name: String,
-min: Calculation,
-max: Calculation,
-values: Array of PointBuyRow,
-cost: Calculation
-total: Calculation,
-pointsLeft: integer,
-spent: integer,
-ignored*: bool

### where PointBuyRow is
-_id: String,
-name: String,
-variableName: String,
-value: integer,
-spent: integer

## Proficiency
-name: String,
-stats: Array of String,
-value: decimal

## Slot (aka propertySlot in the api)
-name: String,
-description*: CalculatedText,
-slotTags: Array of String,
-slotType*: String,
-quantityExpected: Calculation,
-hideWhenFull: bool,
-spaceLeft: integer,
-totalFilled: integer,
-extraTags: Array of ExtraTag,
-unique: String,
-ignored*: bool

## Roll
-name: String, 
-variableName: String,
-roll: Calculation

## Saving Throw
-name*: String,
-target: String,
-stat: String,
-dc: Calculation,
-silent*: bool

## Skill
-name: String,
-skillType: String,
-variableName: String,
-ability: String,
-description*: CalculatedText,
-abilityMod: integer,
-proficiency: decimal,
-value: integer,
-baseValue*: Calculation,
-baseProficiency*: decimal

## Slotfiller
-name: String, 
-slotQuantityFilled: integer,
-description: String, 
-picture*: String,
-slotFillerType*: String,
-slotFillerCondition*: String

## Spell List
-name: String, 
-maxPrepared*: Calculation,
-dc*: Calculation,
-attack_roll_bonus*: Calculation,
-description*: CalculatedText,
-ability: String,
-abilityMod*: integer

## Spell
-name: String,
-actionType: String,
-target: String,
-resources: Resource,
-castingTime: String,
-duration: String,
-level: integer,
-school: String,
-range: String, 
-verbal: bool,
-somatic: bool,
-material*: String,
-concentration*: bool,
-ritual*: bool,
-description*: CalculatedText,
-summary: CalculatedText,
-prepared: bool,
-alwaysPrepared*: bool,
-deactivatedBySelf*: bool

## Toggle
-name: String,
-condition*: Calculation,
-show_ui*: bool,
-disabled*: bool,
-enabled*: bool,
-deactivatedBySelf*: bool