# include_path
This crate provides an implementation of a [proposed set of macros](https://github.com/rust-lang/rust/issues/75075#issuecomment-922374410) to complement the existing
`include_*`macros in Rust, taking a variadic set of arguments, combining them
into a platform-specific path string at compilation time, and returning
the corresponding underlying macros

You can view examples of usage in the crate documentation.