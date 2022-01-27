# VecGraph

A Graph implemented using nothing but `Vec`s in rust.

## Details

The graph is implemented using two `Vec`s: `nodes` and `edges`.

`nodes` stores "nodes". which are just data points.
`edges` stores `EdgeData`s. Which is just a `Vec` of `usize`. representing all the indices of nodes that a particular node points to. The node and its edges are associated by having the same index. so edges of the node at index 0 are present in the `EdgeData` at index 0.

**For Example**
Let's say we have a graph with four nodes. So we would have four items in the `nodes` `Vec`, and four `EdgeData` items in the `edges` `Vec`.

```
nodes = [Node0, Node1, Node2, Node3]
edges = [EdgeData0 = [], EdgeData1 = [], EdgeData2 = [], EdgeData3 = []]
```

Remember that `EdgeData` is just a `Vec` of `usize`.

Now let's say that Node0 is connected to Node1 and Node3, and that Node3 is connected to Node0 and Node2.
Our Graph will look like this:

```
nodes = [Node0, Node1, Node2, Node3]
edges = [EdgeData0 = [1, 3], EdgeData1 = [], EdgeData2 = [], EdgeData3 = [0, 2]]
```
