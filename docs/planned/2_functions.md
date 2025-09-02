# Functions

### Methods
Methods are functions defined on the entity instance level. Rumil doesn't have user-defined non-method functions. Rumil has three classes of methods determined by their transitivity. Transitivity is simply a way to describe what a verb or method acts "upon" vs what it acts "with". In natural language, we specify these entities as "direct" and "indirect" objects. In terms of computer programming, this corresponds to mutability. Transitivity only aplies to *reference types* (entities and collections); primitives are *value types*, so they are always passed as copies. These are Rumil's method classes:

### Transitivity Classes
* **Intransitive**: a non-mutating function that can take zero or more immutable arguments (indirect objects). Indirect objects are listed in the parentheses of a function signature.
* **Transitive**: a mutating function that must take at least one mutable argument (direct object) and can take zero or more immutable arguments. The direct objects are listed after the parentheses of a function signature and immediately preceded by the director operator `|>`. An argument must be a direct object even if it's only passed to another transitive method but not directly mutated by the method.
* **Reflexive**: a transitive method where the first direct object is implicitly bound to "this" instance of the entity type that contains the method. A method is declared reflexive with the reflexor operator `@` at the beginning of the method name (but following the privator, if present).

### Return Values and Method Signatures
A method can return zero or more values. The return type is declared using a type annotation between the method name and parentheses. If there is no return type, the annotation is not required. To return a value from inside the method body, the resultor operator `=:` is used.

Here is an example method signature that incorporates all components. Only the name and parentheses are universally required:

`_@copy_name<b> (title<s>) |> whom<Person> {...}`

This signature describes a private reflexive method that takes a string arg called `title` as an indirect object and an entity arg of type `Person` called `whom` as a direct object, and the method returns a boolean value.

### Constructors, Destructors and Presentors
Entity types have three special methods called the *constructor*, *destructor*, and *presentor*, which initialize, tear down, and return a string representation of an instance, respectively.

Constructors are declared using the `+` symbol as the method name, and destructors are declared using the `-` symbol as the method name. Neither method can be made private and both are optional but useful to implement. Any amount of constructors or destructors can be implemented, as long as they have unique arguments. There is always a default (no-arg) implementation that can be overridden. Presentors are declared using the `$` symbol as the method name, and it can also not be made private. However, there is only one valid signature, taking no arguments. The default implementation simply returns the instance variable's name and type.
These methods are not called directly like typical methods. Here is how to use them:

**Constructors**
* Implement a constructor with any amount of indirect object arguments, ie, in type `Person`: `+(name<s>) {.name = name;}`
* Declare a variable to serve as the reference for the instance, ie: `bob<Person>;`
* When binding or assigning the variable, call the constructor by calling the type name as a function with the arguments for the constructor, ie: `bob = Person("Bob");`

**Main Type Constructors**
* The main type has two constructor signatures that can be implemented:
  * `+()`: no arguments, called when program is invoked without command line arguments
  * `+(args <[s]>)`: one string list as an argument, called when program is invoked with command line arguments
  * The main type's constructor can not be called directly, ie, this code is invalid: `main<*> := *();`

**Destructors**
* Implement a destructor with any amount of indirect object arguments, ie, in type `Person`: `-(farewell<s>) {$v(farewell);}`
* When the last reference to an entity goes out of scope, its default destructor is called (no arguments in signature)
* You can also explicitly destroy the entity behind a reference by calling a destructor via the dispose function, ie: `-("Goodbye world!") |> bob;`
  * The variable `bob` is now a nil reference: `ø`
* Destructors are useful for cleanup logic such as closing unmanaged resources like file handlers and external client connections, or simply for logging lifetime events for entities.

**Presentors**
* Implement a presentor; it must take zero arguments and return a string, ie in type `Person`: ```$<s>() { =: `Person named {.name}`; }```
* Presentors are automatically called on instances of any type when they are passed as arguments to the echo function or cast to a string.

### Functors
Functors are unnamed methods that allow an entity instance to be called directly like a function. An entity type may define any number of functors, as long as they have unique arguments. Functors can be transitive, intransitive, or reflexive.

### Built-in Functions
Rumil provides the following built-in functions:
* **Echo**: `$` - intransitive, takes any argument that is predicated on `s` (see **Predication and Typecasting**) and prints it to stdout. Its symbol is reminiscent of a Unix shell prompt.
  * `$("Hello, world!");` 
    * Output: `Hello, world!`
  * `$(client);`
    * Output: `client<Client>`
    * In this example, `client` is an instance of `Client`, which has no presentor implementation
* **Echo (newline)**: `$v` - Identical to `$`, but always appends a newline suffix (`\n`)
* **Error**: `!` - same as `$`, but writes to stderr.
  * `!("Something went wrong");`
* **Exit**: `,` - intransitive, takes an integer error code to return from the execution and exits the process:
  * `,(1);`
* **Ask**: `:` - transitive, indirect object is the same as `$` and `!`, but its direct object is a string. The indirect object is printed to stdout, and the thread waits for input from stdin, which is written to the direct object:
  * `:("What is your name?\n: ") |> name;`
* **Size**: `#` - intransitive, takes any variable or a type identifier and returns a size measurement that represents something different for different argument types:
  * *Integer, float, and boolean variables*: number of **bits** needed to store its current value
  * *Char and entity variables*: number of **bytes** needed to store its current value
  * *String and collection variables*: number of chars in the string or elements in the collection
  * *Data types*: number of bytes required to **instantiate** the type in memory
* **Dispose**: `-` - transitive, indirect objects are destructor arguments for an entity type, and direct object is an instance of that type. It detects the entity's type and calls the destructor with the matching signature if it exists.
  * `-() |> client;`
* **Clone**: `&` - itransitive, indirect object is an entity instance to clone. Returns a new instance of the same type as a deepcopy of the indirect object.
  * `bob2<Person> := &(bob);`