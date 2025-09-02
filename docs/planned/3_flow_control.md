# Flow Control

### Branching
Branching is done using the branchor operator `?` between boolean expressions (predicates) and code blocks (consequents). This gives us the familiar "if-elseif-else" structure. It is important to note that the only difference between the introduction of an "elseif" block and a new independent "if" block is whether the branchor expression is on the same line as the closing brace of the previous code block. Here are some examples that build upon eachother:

**If**
```
num<i> := 10;
num % 5 == 0 ? {
    $v(`{num} is divisible by 5`);
}
```
Output:
```
10 is divisible by 5
```

<br>

**If-else**
```
num<i> := 10;
num % 3 == 0 ? {
    $v(`{num} is divisible by 3`);
} ? {
    $v(`{num} is not divisible by 3`);
}
```
Output:
```
10 is not divisible by 3
```

<br>

**If-elseif-else**
```
num<i> := 11;
num % 2 == 0 ? {
    $v(`{num} is an even number`);
} num % 3 == 0 ? {
    $v(`{num} is an odd multiple of 3`);
} ? {
    $v(`{num} is an odd number not divisible by 3`);
}
```
Output:
```
11 is an odd number not divisible by 3
```

<br>

**If, if**
```
num<i> := 10;
num % 2 == 0 ? {
    $v(`{num} is an even number`);
}
num > 6 ? {
    $v(`{num} is greater than 6`);
}
```
Output:
```
10 is an even number
10 is greater than 6
```