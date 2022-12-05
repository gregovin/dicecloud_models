First, we have PropVal, which can be an integer, decimal, bool, or string

### ValWrap
- value: PropVal

## Calculation
- calculation: String,
- _key: String,
- type: String,
- hash: integer,
- parse_node: ParseNode,(will deal with this at the end as it gets into AST stuff)
- errors: Array of ParseError,
- value: PropVal

### ParseError
- type: String,
- pub message: String

## ExtendedCalc
same as Calculation, but with the following fields:
- baseValue*: PropVal,
- effects*: Array of Effect

## CalculatedText
- text: String,
- value: String,
- hash: integer,
- inlineCalculations: Array of Calculation

## Effect
- _id: String,
- name: String,
- operation: String,
- amount:ValWrap,
- type*: String

### ParseNode has a parseType which determines which fields it has
#### parseType="accessor"
- path: Array of String,
- name: String
#### parseType="array"
- values: Array of ParseNode

#### parseType="call"
- functionName: String,
- args: Array of ParseNode

#### parseType="constant"
- valueType: String,
- value: PropVal

#### parseType="if"
- condition: ParseNode,
- consequent: ParseNode,
- alternative: ParseNode

#### parseType="index"
- array: ParseNode,
- index: ParseNode

#### parseType="operator"
- left: ParseNode,
- right: ParseNode,
- operator: String,
- fn: String

#### parseType="not"
- right: ParseNode

#### parseType="parenthesis"
- content: ParseNode

#### parseType="roll"
- left: ParseNode,
- right: ParseNode

#### parseType="symbol"
- name: String

#### parseType="unaryOperator"
- operator: String,
- right: ParseNode