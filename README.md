# RedBlack tree baked by a vector

The same storage can be used by multiple trees. This allows nodes to travel from one tree to another without relocation.
Storing the nodes in a vec results in good cache locality. This datastructure is designed for hundred-housands of nodes and move nodes from one tree into another.

Fuzz-tested to assure the tree always respects RB rules. 