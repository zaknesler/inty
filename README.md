### inty

Basic interpreter written in Rust, not following any particular guides or languages. This is just a fun little project, and I'm probably doing things incorrectly. The tests pass.

#### Features

- [x] Arithmetic, e.g. `3 + 4 * 2`
- [x] Boolean operations, e.g. `true || !false`
- [x] Relational operations, e.g. `4 >= 3`
- [x] Variable assignment, e.g. `let x = 3`
- [x] Scoping, e.g. `{ let x = 3; x } => 3`
- [x] Shadowing, e.g. `{ let x = 3; { let x = 4 }; x } => 3`
