# DKG tooling

This is a fork of the Celo's [threshold-bls](https://github.com/celo-org/celo-threshold-bls-rs) project.

## Overview

This project provides tooling for running an interactive distributed key generation protocol.  
All participants (administrator and key holders) use a command line app to participate in the protocol, and all the communication is hapenning through a smart contract.  

In the beginning of the DKG procedure:
- The list of account addresses belonging to key holders is publicly known
- There is a designated administrator that deploys and initialized the contract

In the result of the DKG procedure:
- All key holders have their secret key share locally
- Individual public key shares and master public key are available publicly

This project implements the JF-DKG scheme described in [Secure Distributed Key Generation for Discrete-Log Based Cryptosystems
](https://link.springer.com/article/10.1007/s00145-006-0347-3)

## Install

Get latest binaries from the [releases](https://github.com/trilitech/dkg-tooling/releases) page.

Alternatively use Docker images:
```
docker run ghcr.io/trilitech/dkg-cli:$RELEASE_TAG -h
```

## Use

Check out the [instructions](crates/dkg-cli).

## Build from sources

Build with `NO_SOLC_BUILD=1 cargo build --release`.

All crates require Rust 2021 edition and are tested on the following channels:
- `1.76.0`

If you do not have Rust installed, run: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Disclaimers

**This software has not been audited. Use at your own risk.**
