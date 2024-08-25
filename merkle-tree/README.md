Messy (but working) implementation of MerkleTree data structure.

TODO:

- improve existing impl to reduce borrow checker workarounds (is Rc right
  approach?)
- implement version based only on vector, where each three nodes has its index
  based on the tree position
    - size of each vector item can be predictable (stack based) 
