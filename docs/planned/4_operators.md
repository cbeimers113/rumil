# Operators

### Mathematical Operators
| Symbol | Name | Unary/Binary | Description | Types |
|---|---|---|---|---|
| `+` | *add* | **B** | Sums its operands | `i`, `f` |
| `-` | *negate* | **U** | Swaps the sign of its operand | `i`, `f` |
| `-` | *subtract* | **B** | Subtracts RH from LH | `i`, `f` |
| `*` | *multiply* | **B** | Multiplies its operands | `i`, `f` |
| `/` | *divide* | **B** | Divides LH by RH | `i`, `f` |
| `**` | *exponentiate* | **B** | Raises LH to the power of RH | `i`, `f` |
| `%` | *modulo* | **B** | Returns the remainder of the floor division of LH by RH\* | `i` |
| `_` | *floor* | **U** | Floors its operand | `f` |
| `_` | *floor divide* | **B** | Floors the quotient of LH by RH | `f` |
| `\|` | *magnitude* | **U** (enclosing) | Returns the absolute value of its operand | `i`, `f` |
\* Modulo is the remainder operator, not the true modulus

### Bitwise Operators
Bitwise operators are only applicable to the `i` type. They are all also usable as mutators.

| Symbol | Name | Unary/Binary | Description |
|---|---|---|---|
| `!` | *NOT* | **U** | Flips the bits of its operand |
| `&` | *AND* | **B** | Returns 1 where the operand bits are both 1 |
| `\|` | *OR* | **B** | Returns 1 where either operand bit is 1 |
| `^` | *XOR* | **B** | Returns 1 where only one operand bit is 1 |
| `<<` | *leftshift* | **B** | Shifts all bits of LH to the left by RH places |
| `>>` | *rightshift* | **B** | Shifts all bits of LH to the right by RH places |

### Mutators
Mutators are operators whose LH operand must be a variable. They correspond to each of the binary mathematical and bitwise operators. The operation is performed on the operands and the result is written to the variable.
| Symbol | Name | Description | Types |
|---|---|---|---|
| `=` | Assignor | Sets value of LH to RH | Any |
| `+=` | Incrementor | Increases value of LH by RH | `i`, `f` |
| `-=` | Decrementor | Decreases value of LH by RH | `i`, `f` |
| `*=` | Multiplior | Multiplies value of LH by RH | `i`, `f` |
| `/=` | Divisor | Divides value of LH by RH | `i`, `f` |
| `**=` | Exponentiator | Exponentiates value of LH by RH | `i`, `f` |
| `%=` | Modulator | Sets value of LH to remainder of floor division of LH by RH | `i` |
| `_=` | Floor Divisor | Divides value of LH by RH then floors | `f` |
| `~=` | Adaptor | Sets value of LH to RH cast to the type of LH (may be lossy; see **Predication**) | Any |

### Computational Operators
Computational operators are for handling instructions rather than performing calculations.
| Symbol | Name | Usage | Example |
|---|---|---|---|
| `:=` | Bindor | Binds an identifier to a value | `num<i> := 3;` |
| `\|>` | Director | Signifies a mutable function argument | (in signature): `add_score (score<f>) \|> whom<Person> {...}` (in call): `add_score(3.5) \|> bob;` |
| `@` | Reflexor | Signifies that "this" instance is mutated by a function | `@add_score (score<f>) {...}` |
| `=:` | Resultor | Returns a value from a function | (ie, sum function): `=: a+b;` |
| `?` | Branchor | Branches logic path conditionally | `num > 5 ? {...}` |
| `.` | Accessor | Gives access to the operand's exposed fields and methods | `$(bob.name);` |
| `^` | Superaccessor | Gives access to the exposed fields and methods of the operand's immediate supertype (if it has one) | `^update();`
| `_` | Privator | Makes its operand privately accessible | `_send_message(msg<s>) {...}` |
| `..?` | Contingor | Repeats a block of code until the condition is met (see **Looping**) | `{n += 1}..? n == 10; ` |
| `..` | Iterator | Repeats a block of for a copy of each element in a collection (see **Looping**) | `person<Person> := ..people {...}` |
| `#..` | Enumerator | Repeats a block of code for a copy of each element and its index in a collection (see **Looping**) | `idx<i>, person<Person> := #..people {...}` |
| `*..` | Addressor | Repeats a block of code for each actual element in a collection of reference types (see **Looping**) | `person<Person> := *..people {...}` |
| `~` | Predicator | *RH operand is a type.* Returns a boolean indicating whether LH is *predicated on* the definition of RH (see **Predication**) | `3.5~<i> ? {...}` |
| `*` | Assertor | Returns a boolean indicating whether an entity is defined (not a nil reference) | `*bob ? {...}` |