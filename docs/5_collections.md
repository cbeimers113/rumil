# Collections

Collections are types that include multiple individual instances of the same type. They are reference types and can hold either primitives, entities, or other collections. They are all dynamically sized. There are three collection types in Rumil: the **list**, the **map**, and the **set**. They are declared with collection notation, and the zero value for each type is the empty collection (`{}`). The **string** primitive can also be considered a type of collection (as a list of `c` values), but it is important to note that strings are value types instead of reference types like the other collections. See the following tables for more information about the collection types and their operations:

| Type | Example | Indexed? | Ordered? |
|---|---|---|---|
| **list** | `names<[s]> := [s]{"Bob", "Alice"};` | Yes; `<i>` | Yes |
| **map** | `scores<[s:f]> := [s:f]{"Bob": 3.5, "Alice": 4.7};` | Yes; any primitive or entity | No |
| **set** | `visited<[s::]> := [s::]{"Toronto", "Washington", "New York"};` | No | No |

### List Operators
| Symbol | Name | Description | Example |
|---|---|---|---|
| `<~` | Inclusor | Returns the first index found of LH within RH if it exists, or -1 | `idx<i> := 6 <~ nums;` |
| `[]` | Indexor | Accesses the element at the given index if in bounds (r/w), or panics | `num<i> := nums[2];` |
| `->` | Insertor | Appends LH to RH | `6 -> nums;` |
| `<:` | Conveyor | Removes and returns (pops) the last element of RH if it exists and binds it to a new variable, or panics | `num<i> <: nums;` |
| `<-` | Withdrawor | Removes and returns (pops) the last element of RH if it exists and assigns it to an existing variable, or panics | `num <- nums;` |

### Map Operators
| Symbol | Name | Description | Example |
|---|---|---|---|
| `<~` | Inclusor | Returns a boolean indicating whether LH is a key in RH | `"Joe" <~ scores ? {...}` |
| `[]` | Indexor | Accesses the value at the given key if it exists (r/w), or panics | `score<f> := scores["Jeff"];` |
| `->` | Insertor | Inserts the LH key-value pair into RH | `"Jeff": 7.2 -> scores;` |
| `<:` | Conveyor | Removes and returns the entry for the given key if it exists and binds it to a new variable, or panics | `score<f> <: scores["Alice"];` |
| `<-` | Withdrawor | Removes and returns the entry for the given key if it exists and assigns it to an existing variable, or panics | `score <- scores["Bob"];` |
| `>-` | Ejector | Removes RH from LH if it exists | `scores >- "Dave";` |

### Set Operators
| Symbol | Name | Description | Example |
|---|---|---|---|
| `<~` | Inclusor | Returns a boolean indicating whether LH is an element in RH | `"Chicago" <~ visited ? {...}` |
| `->` | Insertor | Adds LH to RH | `"Chicago" -> visited;` |
| `>-` | Ejector | Removes RH from LH if it exists | `visited >- "Washington";` |

### String Operators
| Symbol | Name | Description | Types | Example |
|---|---|---|---|---|
| `<~` | Inclusor | Returns the first index found of the beginning of LH within RH if it exists, or -1 | LH ~ `s` | `idx<i> := "el" <~ "hello";` |
| `[]` | Indexor | Accesses the char at the given index if in bounds (r/w), or panics | index ~ `i`, write ~ `s` | `name[0] = 'G';` |
| `->` | Insertor | Appends LH to RH | LH ~ `s` | `" world" -> greeting;` |
| `<:` | Conveyor | Removes and returns (pops) the last char from RH if it exists and binds it to a new variable, or panics | LH is `c` | `final<c> <: name;` |
| `<-` | Withdrawor | Removes and returns (pops) the last char from RH if it exists and assigns it to an existing variable, or panics | LH is `c` | `final <- name;` |

### Operators from Set Theory
The following operators are applicable to sets and maps. They operate on the set itself or the keyset of a map. They are also all usable in mutator form.
| Symbol | Name | Description | Example |
|---|---|---|---|
| `\|` | Unionor | Creates a new collection with all elements present in either operand | `all<[s::]> := left \| right;` |
| `&` | Intersector | Creates a new collection containing only the elements present in both operands | `both<[s::]> := left & right;` |
| `^` | Exclusor | Creates a new collection containing only elements that aren't present in both operands | `unique<[s::]> := left ^ right;` |
| `-` | Subtractor | Creates a new element containing all elements of LH not present in RH | `diff<[s::]> := left - right;` |