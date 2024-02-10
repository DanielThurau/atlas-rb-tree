# Red-black Tree in Rust

This project is a Rust implementation of the Red-black Tree
data structure as described in __Introduction to Algorithms 4th edition__. 
You Could call this a "textbook implementation". 

A Red-black Tree is a kind of self-balancing binary search tree.
Each node of the tree has an extra bit for denoting the color 
of the node, either red or black. A Red-black Tree ensures a 
balanced tree by enforcing certain rules through rotations and
color changes of nodes, which in turn guarantees `O(log n)` time
complexity for search, insertion, and deletion operations.

## Usage
Here's a quick example of how to use the Red-black Tree to 
insert elements and search within the tree:

```rust
use rb_tree::Tree;

fn main() {
    let mut tree = Tree::new(0); 
    tree.insert(5);
    tree.insert(3);
    tree.insert(10);
    if tree.contains_key(5) {
        println!("Found 5 in the tree!");
    }
    
    tree.delete(5);
}
```

## Running the tests

```bash
cargo test
```

## Running the benchmarks

```bash
cargo bench
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.MD) for details on our code of conduct, and the process for submitting pull requests to us.

## License
This project is licensed under the MIT License - see the [LICENSE.txt](LICENSE.txt) file for details.


## TODOS

- ✅ Implement the standard APIs of a tree
- Final unit testing + integration testing
- ✅ Basic Benchmarks
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

