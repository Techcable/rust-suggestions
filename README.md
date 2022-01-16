suggestions
===========
Minimal Rust library to provide clap-style "Did you mean?" suggestions

The only dependency is [strsim](https://lib.rs/crates/strsim).

The implementation is copied directly from clap ([see here](https://github.com/clap-rs/clap/blob/7b7c76e3d0279b474c774ea738aecb1d77251df8/src/parse/features/suggestions.rs#L12-L24)). It has just been extracted into a library.

## Binary
A binary is available as an example of how to use the library.

It has no additional dependencies. Desired targets are provided as arguments, and "possible strings" are read from standard input

### Examples

