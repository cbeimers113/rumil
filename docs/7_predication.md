# Predication

Predication allows the programmer to quickly determine if a field satisfies the constraints of another type. That is, it tells us if a field can be safely casted to another type (and therefore back) without loss of data. The predicator operator `~` takes a field as the LH operand and a type as the RH operand and evaluates to a boolean value. For primitives and collections, the operator returns whether the current *value* of LH can be represented as the type on RH. For entities, it returns whether LH is an instance of RH or if LH is a supertype of RH and can be casted to RH without any field or method overrides. See the table below for more information on the predication rules for primitives and collections.

#### Predication Rules
| LH type | RH type | Predicate |
|---|---|---|
| Any | Same as LH | tautology |
| `i` | `b` | true when LH is 0 or 1 |
| `i` | `f` | true when LH is exactly representable as a valid signed 64-bit floating point number |
| `i` | `c` | true when LH is a valid scalar Unicode point |
| `i` | `s` | tautology |
| `i` | Any collection | true when the collection value type is `i` |
| `b` | `i` | tautology |
| `b` | `f` | tautology |
| `b` | `c` | tautology; `U+0000` and `U+0001` are non-printable control codes, but still valid code points |
| `b` | `s` | tautology |
| `b` | Any collection | true when the collection value type is `b` |
| `f` | `i` | true when LH is storing an integer |
| `f` | `b` | true when LH is 0.0 or 1.0 |
| `f` | `c` | true when LH is predicated on `i` and LH as `i` is predicated on `c` |
| `f` | `s` | tautology |
| `f` | Any collection | true when the collection value type is `f` |
| `c` | `i` | tautology |
| `c` | `f` | tautology |
| `c` | `b` | true when LH is `U+0000` or `U+0001` |
| `c` | `s` | tautology |
| `c` | Any collection | true when the collection value type is `c` |
| `s` | `i` | true when LH is a string representation of a valid signed 64-bit integer, including binary, hex, and octal prefixed forms |
| `s` | `f` | true when LH a string representation of a valid signed 64-bit floating point number |
| `s` | `b` | true when LH is either "1" or "0" with any amount of "0"s preceding, including binary, hex, and octal prefixed forms |
| `s` | `c` | true when LH contains a single Unicode point |
| `s` | *list* or *map* | true when the collection value type is `s`, `c`, `i` or `f` |
| `s` | *set* | true when the collection value type is `s` |
| *list* | *map* | tautology |
| *list* | *set* | true when LH has 0 or 1 elements |
| *map* | *list* | true when LH's keyset contains consecutive values predicated on `i`, starting from 0 |
| *map* | *set* | true when LH has 0 elements or has one element with key predicated on `i` and equal to 0  |
| *set* | *list* | true when LH has 0 or 1 elements |
| *set* | *map* | true when LH has 0 or 1 elements |
