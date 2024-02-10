Hash Tree Crypt
===========================

Hash tree crypt is an seekable stream cipher using a perfect binary tree to provide an infinite (quite large) one-time pad.

## Principle

The user password is hashed, this hash become the root of the tree.

The tree is 32 levels deep, each level is:
  - left child: the hash of the parent hash
  - right child: the hash of the parent reversed-hash

We don't really use a tree, but a list of the hashes used from the root to the current leaf.

So we have 2^32 hashes that we can use to encrypt the file, and we can seek to any position in the file with at most 32 hashes to compute.

The maximum file size is therefore 2^32 * 32 bytes, which is 128 GB.

## Usage

### Build
```bash
cargo build --release
```

### Clean
```bash
cargo clean
```

### Run
```bash

# Development
cargo run --release -- -i file.txt -o file.txt.htcrypt

# Encryption and Decryption are the same
hash-tree-crypt -i file.txt -o file.txt.htcrypt
hash-tree-crypt -i file.htcrypt -o file.txt

# Output option is optional
hash-tree-crypt -i file.txt

# Input option is optional when using stdin
cat file.txt | hash-tree-crypt

# Output can be stdout (but --output must not be used, otherwise stdout will be empty)
hash-tree-crypt -i file.txt > file.txt.htcrypt
```

### Install
```bash
cargo install --path .
```
