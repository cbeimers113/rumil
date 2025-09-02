# Looping

Rumil's looping structures are similar to other languages. The equivalent of "do while" loops in other languages is the "do until" (*contingent*) loop in Rumil. There are also versions of iterative ("for") loops that correspond to counting through a range (*consecutors*) or iterating over each element in a collection, with (*iterators*) or without (*enumerators*) an index. Iterations over reference-type collections can also be done by value (*surveyors*).

### Contingent Loops ("Do Until")
The difference between a conventional *while* loop and a *contingent* loop is that the structure is essentially inverted; *while* loops state the condition that must be true in order for the body to execute, whereas *contingent* loops state the condition that must be true in order for execution to **stop**. Contingent loops also state their stop condition (the *contingency*) after the loop body via the consecutor operator (`..?`).
See the following example comparing a typical C *while* loop with the equivalent Rumil *contingent* loop.

**C**
``` C
int n = 0;
while n < 10 {
	n++;
}
```

**Rumil**
```
n<i> := 0;
{
	n += 1;
}..? n == 10;
```

### Iterative Loops
Iterative loops are constructed using the iterator operator (`..`). Each element in a collection is given *by value* to the scope of the loop block. Ordered collections and strings have stable loop ordering, whereas unordered collections do not have a stable looping order. 
The following example loops over a list of names and prints each one:
```
names<[s]> := [s]{"Bob", "Alice", "Jeff", "Mary"};

$v("Names:");
name<s> := ..names {
	$v(name);
}
```

**Consecutor**:
A consecutor is the equivalent of the familiar `for i = 0; i < n; i++` looping structure in many other languages. It uses range notation (which Rumil includes as a shorthand to create an integer list) with the iterator operator to provide a local variable for each consecutive integer in the range. See the following example comparing C to Rumil once again.

**C**
``` C
for(int n = 0; n < 10; n++) {
	printf("n = %d\n", n);
}
```

**Rumil**
```
n<i> := ..[0, 10) {
	$v(`i = {i}`);
}
```

**Enumerator:**
The enumerator (`#..`) works the same way as the iterator on its own, but it also provides a value for the index of each element in the collection. It is only applicable to indexed types (strings, maps, lists). The index variable for a string or list is `i`, and for a map it is the map's key type. Here is an example iterating over a map of names to scores:
```
scores<[s:f]> := [s:f]{
	"Dave": 5.0,
	"Alice": 6.1,
	"Bob": 4.7,
}

name<s>, score<f> := #..scores {
	$v(`{name}'s score was {score}`);
}
```

**Addressor:**
The addressor (`*..`) is also very similar to the iterator, except it provides access to the actual elements in the collection being iterated over. It is therefore only applicable to collections of reference types. The ability to actually mutate the data behind these references is still subject to the transitivity of the collection if it is an argument variable (*a direct object collection is a collection of direct objects*). See this example of a transitive function that takes a list of `Person` entities and adds a prefix to each of their names:
```
graduate() |> people<[Person]> {
	person<Person> := *..people {
		person.name = `Dr. {person.name}`;
	}
}
```
