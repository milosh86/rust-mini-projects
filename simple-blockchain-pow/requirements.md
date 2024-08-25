# Mini Blockchain Requirements

## Project Overview

The mini blockchain project aims to implement a simplified version of a blockchain system in Scala. The project should
demonstrate key blockchain concepts such as blocks, transactions, Proof-of-Work, mining, and validation.

## Features

The following features should be implemented in the mini blockchain project:

1. Block Structure:
    - Each block should contain an index, parent hash, transactions, mining target number, and nonce.
    - The cryptographic hash of a block should be calculated using the SHA-256 algorithm.

2. Mining:
    - Blocks should be mined by finding a valid nonce that results in a hash below the mining target number.
    - The mining process should follow a proof-of-work mechanism.
    - The target number should be adjustable, allowing the simulation of different mining difficulties.

3. Block Validation:
    - Blocks should be validated by ensuring that their hash is below the mining target number.
    - Validated blocks should be appended to the blockchain.

4. Blockchain:
    - The blockchain should maintain a sequence of blocks linked together by hashes.
    - The blockchain should support appending new blocks and retrieving blocks by index or hash.
    - A common ancestor block between two blockchains should be discoverable.

5. Proof-of-Work or PoW for short:
    - A computationally intensive algorithm whose role is to solve a puzzle that allows the next block to appear in the
      blockchain.

6. Genesis Block:
    - The project includes a predefined Genesis block, representing the first block in the blockchain.

## Development Guidelines

- The project should follow good coding practices, including proper code organization, naming conventions, and code
  documentation.
- Unit tests should be written to ensure the correctness of the implemented functionalities.
- The code should be modular and easily extendable, allowing future enhancements and additions.

## Documentation

- The project includes a `README.md` file that provides an overview of the project, instructions for building and
  running the code, and any additional information deemed necessary.

