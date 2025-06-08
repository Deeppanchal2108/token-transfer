# Token Transfer Program (Solana + Anchor)

This is a simple Solana smart contract built with Anchor that transfers SPL tokens between two accounts securely.

## Features

- Verifies signer identity
- Validates mint consistency
- Performs token transfer using CPI
- Handles custom errors

## Stack

- Solana
- Anchor
- Rust
- SPL Token Program (via anchor-spl)

## Build & Test

```bash
anchor build
anchor test
