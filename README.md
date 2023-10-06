### inty

Basic interpreter written in Rust, not following any particular guides or languages. This is just a fun little project and is not too serious, but all the tests pass. It uses a basic recursive-descent parser, but I would like to rewrite it using a grammar such as `pest.rs` or a parser combinator like `nom`.

#### Features

- [x] Arithmetic, e.g. `3 + 4 * 2`
- [x] Boolean operations, e.g. `true || !false`
- [x] Relational operations, e.g. `4 >= 3`
- [x] Variable assignment, e.g. `let x = 3`
- [x] Scoping, e.g. `{ let x = 3; x } => 3`
- [x] Shadowing, e.g. `{ let x = 3; { let x = 4 }; x } => 3`
- [x] Branching, e.g. `if 4 >= 3 then { 1 } else { 2 }`
- [x] Arrays, e.g. `[0, 1, 2, 3]`

#### Eventually

- [ ] Variable re-assignment, e.g. `{ let x = 3; x = 4; x } => 4`
- [ ] Pre/post-fix operations, e.g. `{ let x = 1; x++; --x; x += 2; x } => 3`
- [ ] Loops, e.g. `for x in [0, 1, 2, 3] { x }` or `for x = 0; x <= 5; x += 1 { x }`
- [ ] Functions, e.g. `{ fn foo(x) { x + 3 }; foo(1) } => 4`
