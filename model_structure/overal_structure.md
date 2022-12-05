Properties with a * are optional
# A character in dicecloud has the following properties(every time)
- creatures: an array of CreatureInfo
- creatureProperties: an array of FlatProp
- creatureVariables: an array of CreatureVariables

## CreatureInfo consists of
- id: String,
- owner: String,
- name: String,
- gender: String,
- alignment: String,
- allowedLibraries: array of String
- allowedLibrary_collections: array of String
- deathSave: DeathSaveInfo,
- denormalizedStats: DenormalizedStats,
- type: String,
//damageMultipliers and variables properties omitted
- settings*: Settings
- readers: array of String
- writers: array of String
- public: bool,
- picture*: String,
- avatarPicture*: String

### DeathSaveInfo consists of
- pass: integer(unsigned),
- fail: integer(unsigned),
- canDeathSave*: bool,
- stable: bool

### DenormalizedStats
- xp: integer(unsigned),
- milestoneLevels*: integer(unsigned)

### Settings
- showTreeTab*: bool,
- hideRestButtons*: bool,
- hideUnusedStats*: bool,
- hideSpellsTab*: bool