# Inheritance

Rumil provides basic support for inheritance and polymorphism. A type can be declared as a subtype of another using a type annotation on the type name. The main type cannot be extended. For instance, to create a type called `Employee` that extends `Person`:

```
Employee<Person> {
    ...
}
```

### Superconstructors and Superdestructors
Constructors and destructors of subtypes must immediately call the corresponding method for the supertype in every implemented constructor or destructor body. The superaccessor operator `^` is used to call superconstructors and superdestructors. Ie;

```
Person {
    name<s>;

    +(name<s>) {
        .name = name;
    }

    -(farewell<s>) {
        $v(farewell);
    }
}

Employee<Person> {
    position<s>;

    +(name<s>, position<s>) {
        ^+(name);
        .position = position;
    }

    -() {
        ^-(`Goodbye from {.name}! I was a {.position}`);
    }
}
```

### The Superaccessor
The superaccessor `^` gives access to the exposed fields and methods of the immediate supertype, if an entity type has one. It can be used without an identifier to call a superfunctor.