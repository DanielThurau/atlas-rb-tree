# rb-tree

## Overview

Repo for implementing a Red-black Tree.

Based on __Introduction to Algorithms__ 4th edition Red-Black Trees.


## TODOS

- ✅ Implement the standard APIs of a tree
- Final unit testing + integration testing
- Benchmarks
- Review releasing libraries into crates.io
- Review rust Library API guidelines (https://rust-lang.github.io/api-guidelines/about.html) & https://protobuf.dev/programming-guides/api/#use-different-messages
- Rust testing layout (see my notion doc @Daniel)
- Review and administer to my project
  - https://matklad.github.io/2021/02/06/ARCHITECTURE.md.html
  - https://matklad.github.io/2021/09/04/fast-rust-builds.html
  - https://matklad.github.io/2021/08/22/large-rust-workspaces.html
  - https://matklad.github.io/2021/07/09/inline-in-rust.html
  - https://matklad.github.io/2021/05/31/how-to-test.html
- Nice GitHub page


### Optimizations
    
- Use a unique node T.nil to represent leaf node children and root parent. This makes the algorithm less memory intensive.

