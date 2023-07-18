# Modules

- **Start from the crate root:** When compiling, the compiler first looks at the root crate. (`src/main.rs` or `src/lib.rs`)
- **Declaring Modules:** Module is declared with `mod` following module name and semicolon: `mod garden;` or `mod garden {}`. The compiler will look for the module's code at these locations:
  - inline, within curly brackets that replaces semicolon
  - in the file `src/garden.ts`
  - in the file `src/garden/mod.rs`
- **Submodules:** Module can also contain submodule. The compiler then again looks at place by the same pattern as it does with regular modules. (`src/garden/fruits.ts`, `src/garden/fruits/mod.rs` and inline in the current module file.)
- **Paths to modules:** If allowed, you can access the local modules by: `crate::garden::fruits`
- **Private vs Public:** As mentioned, you can disallow access to certain modules. By default code within module is private from it's parent modules by default. To make module public, declare it with `pub` keyword (`pub mod garden;`)
- **The `use` keyword:** Just like in `C++` you can shorten the name of namespace. so to access garden and not the type whole namespace path, use `use crate::garden::*`. Just like that you have access to `fruits` without the need to type entire path.
