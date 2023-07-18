# Packages and Crates

## Crates

A crate is smallest amount of code that the rust compiler considers at a time.
Crate can come in two forms: A binary crate or library crate.

Binary are already compiled programs.

Library crates don't have `main` function, and they don't compile to an executable. Instead they fine functionality intended to be shared with multiple projects. (Something like `.so`, `.dll`)

Crate root is a file, that the compiler looks at first. (Usually `src/main.rs` for binary and `src/lib.rs` for library)

## Package

package is a bundle of one or more crates, that provides a set of functionality. Package is directed by `Cargo.toml` file. Package can contain as many of binary crates as you like, but can contain only one library crate.
