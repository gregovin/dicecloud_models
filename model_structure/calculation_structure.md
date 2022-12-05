## PropVal
can take on a value of either a bool, number, decimal, or integer

## ValWrap
- value: [PropVal](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#propval)

## Calculation
- calculation: String,
- _key: String,
- type: String,
- hash: integer,
- parse_node: [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode),
- errors: Array of [ParseError](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parseerror),
- value: PropVal

### ParseError
- type: String,
- pub message: String

## ExtendedCalc
same as [Calculation](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#calculation), but with the following fields:
- baseValue*: [PropVal](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#propval),
- effects*: Array of [Effect](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/additional_types.md#effect)

## ParseNode
has a parseType which determines which fields it has
### parseType="accessor"
- path: Array of String,
- name: String
### parseType="array"
- values: Array of [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode)

### parseType="call"
- functionName: String,
- args: Array of [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode)

### parseType="constant"
- valueType: String,
- value: PropVal

### parseType="if"
- condition: [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode),
- consequent: [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode),
- alternative: [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode)

### parseType="index"
- array: [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode),
- index: [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode)

### parseType="operator"
- left: [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode),
- right: [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode),
- operator: String,
- fn: String

### parseType="not"
- right: [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode)

### parseType="parenthesis"
- content: [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode)

### parseType="roll"
- left: [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode),
- right: [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode)

### parseType="symbol"
- name: String

### parseType="unaryOperator"
- operator: String,
- right: [ParseNode](https://github.com/gregovin/dicecloud_models/blob/master/model_structure/calculation_structure.md#parsenode)