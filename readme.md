This library is designed to be used to deal with data recieved from the dicecloud v2 API.
For example,
```
use dicecloud_models::datamodels::flat_models
//suppose we have the data in a string char_json
let character = FlatCharacter::from_str(&char_json);

```