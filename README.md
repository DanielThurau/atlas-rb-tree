# Red-black Tree in Rust

This project is a Rust implementation of the Red-black Tree
data structure as described in __Introduction to Algorithms 4th edition__. 
You could call this a "textbook implementation". 

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
