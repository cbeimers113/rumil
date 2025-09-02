# Typecasting

Typecasts can be attempted on any field, as long as primitive is cast to primitive and entity is cast to entity. Whether a particular field's value is predicated on the type being cast to has no impact on whether the cast can be performed, only whether it will succeed without data loss. It can be useful to guard typecasts with a predicate check using the predicator. There are two ways to perform a typecast: via the adaptor or with a type annotation:

```
// Typecasting with the adaptor:
@update_scores (score<f>, update<b>) {
    highscore<i> := .get_highscore();
    
    update ? {
        highscore ~= score;
        .set_highscore(highscore);
    }
}

// Typecasting with a type annotation:
@add_score (score<f>) {
    current<i> := .get_score();
    .set_score(current + score<i>);
}
```

### Unpredicated Typecasts
Here is a breakdown of what happens when a typecast is unpredicted:
| LH type | RH type | Result |
|---|---|---|
| `i` | `b` | 1; non-zero integers are truthy |
| `i` | `f` | Rounded to the nearest representable float value |
| `i` | `c` | Either set to the Unicode point represented by the 32 least-significant bits of LH (if valid), or panics |
| `i` | Any collection | Panics |
| `b` | Any collection | Panics |
| `f` | `i` | Set to LH with fractional value truncated |
| `f` | `b` | 1; non-zero floats are truthy |
| `f` | `c` | Either set to the Unicode point represented by the 32 least-significant bits of LH as an integer (if valid), or panics |
| `f` | Any collection | Panics |
| `c` | `b` | 1; non-zero chars are truthy |
| `c` | Any collection | Panics |
| `s` | Any primitive | Panics |
| `s` | *list* or *map* | Panics |
| `s` | *set* | Either creates a set of all unique chars in the string if the set type is `c`, `i` or `f` or panics |
| *list* | *set* | Creates a set of all unique elements in the list |
| *map* | *list* | Panics |
| *map* | *set* | Creates a set from the values in the map |
| *set* | *list* | Creates a list of values from the set, but with unstable ordering |
| *set* | *map* | Creates a map of values from the set, but with unstable key set |
| *entity* | *entity* | Panics