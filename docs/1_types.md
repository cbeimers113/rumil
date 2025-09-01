# Types

### Entities
Rumil's foundational data type is the *entity*.  Entity types are declared by providing a name with a capital letter, followed by a set of curly braces to contain the body:

```
Person {
    ...
}
```

Each program must contain the main type, which is named with an asterisk. It is instantiated once and contains the entry point of the program:
```
* {
    ...
}
```

### Primitive Data Types
There are five primitive data types in Rumil, and they are referenced inside type annotations (`<>`) with a single character type identifier (this is the closest Rumil comes to implementing true "keywords"):
* **Integer**: `i` - a 64 bit signed integer
* **Float**: `f` - a 64 bit signed floating point number
* **Bool**: `b` - a one bit boolean flag
* **Char**: `c` - a single 32-bit Unicode point
* **String**: `s` - a string of Unicode points

### Variables and Constants
To declare a variable, state the name, add a type annotation with the `<>` brackets, and optionally assign an initial value using the bindor `:=`. For example, here we declare a string variable called `name` with no initial value, and an integer variable called `age` with an initial value of 25:
```
name<s>;
age<i> := 25;
```

To update the value of a variable, we can use any of the mutator operators that are familiar from many other languages, such as the assignor: `=`, incrementor: `+=`, and modulator: `%=`. The bindor can only be used once on a field, and must be used at declaration time if at all. If the bindor is not used at declaration time, the variable is implicitly bound to a default zero-value. (Rumil uses a vocabulary of "-or" suffixed terms to describe its operators).

To create a constant, simply name the field with a capital letter. The bindor must be used at declaration time, and mutators can't be used on constants:
`PI<f> := 3.14159;`

String literals are enclosed with double quotes (`"hello"`), and formatted strings can be produced with backticks and braces for variable substitution (``` `My name is {.name}` ```).

### Default Values
The default zero-values of the primitive types are:
* `i`: `0`
* `f`: `0.0`
* `b`: `0` (`false`)
* `s`: `""`
* `c`: `U+0000` (`NULL` code point)

The default zero-value of an entity is the nil reference type: `ø`. This type is not used directly in Rumil source code; entity references that go out of scope are automatically destroyed, and the destructor can be used to destroy an entity while it is still in scope.

### Data Access Levels
Rumil has two data access levels that correspond to "public" and "private". These levels can be applied to entity types, fields, and methods. The default data access level is public. To make something private, simply prefix its name with the privator: `_`. The privator has the following impact on different recipients:
* **Types**: type is only accessible inside the same source file.
  * If a type is private, its public members are still accessible outside the type, but only from within the same source file.
* **Fields**: field is only accessible inside the same type.
* **Methods**: method is only callable from inside the same type.
  * Constructors and destructors cannot be made private.
