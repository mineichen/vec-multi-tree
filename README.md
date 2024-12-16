# RedBlack tree baked by a vector

The same storage can be used by multiple trees. This allows nodes to travel from one tree to another without relocation.
Storing the nodes in a vec results in good cache locality. This datastructure is designed for hundred-housands of nodes and move nodes from one tree into another.
A Tree always contains >=1 item

Fuzz-tested to assure the tree always respects RB rules. 

``` rust
use vec_multi_tree::RedBlackTreeSet;

let mut tree = RedBlackTreeSet::new(5);
tree.insert(1);
assert_eq!(vec![&1, &5], tree.iter().collect::<Vec<_>>());
```