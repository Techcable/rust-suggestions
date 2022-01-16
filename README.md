suggestions
===========
Minimal Rust library to provide clap-style "Did you mean?" suggestions

The only dependency is [strsim](https://lib.rs/crates/strsim).

The implementation is copied directly from clap ([see here](https://github.com/clap-rs/clap/blob/7b7c76e3d0279b474c774ea738aecb1d77251df8/src/parse/features/suggestions.rs#L12-L24)). It has just been extracted into a library.

## Examples
```
let possible_vals = vec!["test", "possible", "values"];
let input = "tst";
let suggestions = suggestions::provide_suggestions(input, &possible_vals);
assert_eq!(suggestions, vec!["test"]);
// We have a convenience function to only pick only a single suggestion, giving `Some` or `None`
let single_suggestion = suggestions::provide_a_suggestion(input, &possible_vals);
assert_eq!(single_suggestion.unwrap(), "test");
```

```
let possible_vals = vec!["testing", "tempo"];
let input = "teso"; // Sems ambiguous. Maybe multiple suggestions?
let suggestions = suggestions::provide_suggestions(input, &possible_vals);
// The implementation trys to order matches from "best" to "wort"
assert_eq!(suggestions, vec!["testing", "tempo"]);
// Asking for a single suggestion here would attempt to return the "best" one
```

If nothing is reasonably similar, asking for suggestions
will return `vec![]` or `None`.

```
let possible_vals = vec!["testing", "things", "here"];
let input = "--something-completely_different";
assert_eq!(suggestions::provide_a_suggestion(&input, &possible_vals), None)
```

## Binary
A binary is available as an example of how to use the library.

It has no additional dependencies. Desired targets are provided as arguments, and "possible strings" are read from standard input

### Examples

