### inty

Basic interpreter written in Rust, not following any particular guides or languages. This is just a fun little project and I'm probably doing things incorrectly, but at least the tests pass.

#### Features

- [x] Arithmetic, e.g. `3 + 4 * 2`
- [x] Boolean operations, e.g. `true || !false`
- [x] Relational operations, e.g. `4 >= 3`
- [x] Variable assignment, e.g. `let x = 3`
- [x] Scoping, e.g. `{ let x = 3; x } => 3`
- [x] Shadowing, e.g. `{ let x = 3; { let x = 4 }; x } => 3`
- [x] Branching, e.g. `if 4 >= 3 then { 1 } else { 2 }`

#### Eventually

- [ ] Arrays, e.g. `[0, 1, 2, 3]`
- [ ] Loops, e.g. `for x in [0, 1, 2, 3] { x }`
- [ ] Variable re-assignment, e.g. `{ let x = 3; x = 4; x } => 4`
- [ ] Functions, e.g. `{ fn foo(x) { x + 3 }; foo(1) } => 4`