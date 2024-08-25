[# Mini Blockchain

The mini blockchain project is a simplified implementation of a blockchain system in Scala. It demonstrates key
blockchain concepts such as blocks, transactions, mining, and validation.

## Features

- Block Structure: Each block consists of an index, parent hash, transactions, mining target number, and nonce. The
  cryptographic hash of a block is calculated using the SHA-256 algorithm.
- Mining: Blocks are mined by finding a valid nonce that results in a hash below the mining target number. The mining
  process follows a proof-of-work mechanism, and the target number is adjustable to simulate different mining
  difficulties.
- Block Validation: Blocks are validated by ensuring that their hash is below the mining target number. Validated blocks
  are appended to the blockchain.
- Blockchain: The blockchain maintains a sequence of blocks linked together by hashes. It supports appending new blocks
  and retrieving blocks by index or hash. It also provides functionality to find the common ancestor block between two
  blockchains.
- Genesis Block: The project includes a predefined Genesis block, representing the first block in the blockchain.

## Getting Started

### Prerequisites

- Scala (version 2.1.3)
- SBT (version 1.8.3)

### Installation

1. Clone the repository:

```shell
git clone git@github.com:gillerick/mini-block-chain.git
````

### Building

```shell
cd mini-blockchain 
sbt compile
```

### Testing

```shell
sbt test
```

### Development

The project follows good coding practices, including proper code organization, naming conventions, and code
documentation.
Unit tests are written using the ScalaTest framework to ensure the correctness of the implemented functionalities.
The code is modular and easily extendable, allowing future enhancements and additions.