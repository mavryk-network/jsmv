# Contributing 👩‍💻

Before contributing to `jsmv`, please read these guidelines carefully.

## Overview

`jsmv` is a JavaScript server runtime for Mavryk smart optimistic rollups designed to provide a great developer experience by aiming to be compatible with web standards.

Through `jsmv` developers can set up, deploy and test so called _smart functions_ written in Javascript/Typescript that can get directly executed on the `jsmv` smart rollup node.
It provides a simple interface through which one can deploy smart functions and then call them by sending HTTP requests to a particular _smart function address_.

`jsmv` also provides a local sandboxed environment for developers to test their functions without deploying them to production.

## How it works?

Since smart rollups must compile to WASM, `jsmv` needs to use a JavaScript engine that compiles to WASM - the assembly used for writing Smart Rollups. Therefore `jsmv` is built on _Boa_ - a Javascript engine written in Rust.

In the jsmv_core crates, `jsmv` uses Boa and enables Rust types to be passed around as JavaScript objects. This allows implementation and registration of various APIs written in Rust and their usage as if they were native Javascript objects.

When writing smart functions, we need a way to store data across different calls of the functions. Therefore, `jsmv` _smart functions_ implement a persistent key-value store used for storing and retrieval of arbitrary JSON blobs. This store can be accessed through a global _Kv_ object.

The key-value store implements _optimistic concurrency control scheme_. It is optimistically assumed that conflicts between different transactions (snapshots of the persistent kv store) are sufficiently rare thus not interfering each other. Before commiting, the transaction verifies whether no other transaction has modified the data it has read.

The transactions performed over the KV store offer ACID guarantees and serializability, therefore any transaction can be commited only if it does not conflict with any previously commited ones.

In each transaction, the repeated access to the same key is optimized through caching. Similarly, writes are buffered until the transaction is commited at which point it gets flushed to the persistent KV storage.

`jsmv` implements several `jsmv`-specific APIs such as `Kv`, `Ledger`, and `Contract`. Additionally, `jsmv` provides implementations for many web standard APIs in the `jsmv_api` crate.

## `jsmv`-specific APIs

### KV store

_Kv_ store is implemented on top of jsmv\*core::kv. The API provides access to a persistent key-value database that can be used to store and retrieve JSON blobs built directly into the jsmv runtime via a global _Kv_ object.

### Ledger

A specialised type of the KV store is the Ledger that provides access to the balances of the L2 tez. Additionally it also stores so-called 'self address' - the address of the smart function itself. Similarly to the KV store, all operations on the ledger are synchronous and atomic, commited only if the request to the smart function succeeds.

### Contract

<!-- TODO Contract -->

## Standard APIs

Additionally, `jsmv` provide implementation of many standard web APIs in the `jsmv_api` crate.

<!--//TODO: Explaining how exactly the following works and fits together:

- the APIs get registered to in the Realm that consists of a set of intrinsic objects and global environment
- The Realm wrapper implements various methods for registration and evaluation of different modules, types and host defined objects and handling of context
- JSNative permits Rust types to be passed around as JavaScript objects.
- There is implemented a wrapper over boa engines runtime and also a wrapper over the smart rollup's runtime - erased runtime.
- the APIs use the functionality of the rollup runtime to interact with the blockchain storage and other functionality implemented in jsmv_proto
- jsmv_kernel
-->

## Bridge

In order to transfer ctez from L1 address to an L2 address in `jsmv`, `jsmv` implements a simple ticket-based bridge smart contract built with LIGO. This contract enables users to deposit ctez from an L1 address (`tz1`/`KT1`) to a jsmv address (`tz4`).

## Getting Started

### Setting up your environment 🌿

Install the [Rust](https://rustup.rs/) toolchain:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once `rustup` is installed, the build dependencies can be installed with:

```sh
make build-deps
```

Alternatively, `jsmv` is packaged with Nix. See the [Nix docs](https://nixos.org/download.html) for instructions for your system.
Additionally, ensure [Nix flakes are enabled](https://nixos.wiki/wiki/Flakes#Enable_flakes).

Once Nix is installed, the dev environment can be built with:

```sh
nix develop
```

### Building 👷‍♂️

The `.wasm` file for `jsmv`'s kernel is built with:

```sh
make build
```

You can locate the resulting build artifact at `/target/wasm32-unknown-unknown/release/jsmv_kernel.wasm`.

To build the installer for `jsmv`, execute the following:

```sh
make build-installer
```

### Running `jsmv` locally ⚙️

#### Installing Mavkit 🐙

Our sandbox network uses a custom distribution of Mavkit found [here](https://gitlab.com/mavryk-network/mavryk-protocol/-/tree/6c0621760ddce94afeff3484d9e8a650d8535f25). See the [Mavkit docs](https://protocol.mavryk.org/introduction/howtoget.html?highlight=building#compiling-with-make) for instructions on building Mavkit from source.

Alternative, with Nix, execute the following:

```sh
# Clone Mavkit
git clone git@gitlab.com:mavryk-network/mavryk-protocol.git
cd mavryk-protocol
# Checkout custom distribution
git checkout ole@next-gen@floats
# Build using Nix
nix-build -j auto
```

After Nix successfully builds Mavkit (it takes a long time 🕣), the Mavkit binaries will be accessable from `result`.

Once Mavkit has been built, copy the following binaries to `jsmv`:

- `mavkit-client`
- `mavkit-node`
- `mavkit-smart-rollup-node`
- `mavkit-smart-rollup-wasm-debugger`

### Running the Sandbox 🏖️

You can now start the sandbox with:

```sh
cargo run -- sandbox start
```

This will initially run `mavkit-node` and initialize `mavkit-client`. Once the client is initialized, the `jsmv_kernel` and `jsmv_bridge` is originated and a smart-rollup node is spun up.

## Hacking on `jsmv` 👨‍⚖️

<!-- TODO -->

### `jsmv` Crates

- [**`jsmv_core`**](/jsmv_core) - `jsmv`'s core functionality: host functions, transactional storage, and execution.
- [**`jsmv_api`**](/jsmv_api) - `jsmv`'s JavaScript web standard runtime apis.
- [**`jsmv_kernel`**](/jsmv_kernel) - `jsmv`'s smart rollup kernel, compiled to WASM.
- [**`jsmv_crypto`**](/jsmv_crypto) - `jsmv`'s crypto library. Primarily a wrapper around `mavryk_crypto_rs`.
- [**`jsmv_proto`**](/jsmv_proto) - `jsmv`'s protocol: `jsmv` specific runtime apis, storage context, execution of operations.
- [**`jsmv_cli`**](/jsmv_cli) - `jsmv`'s client CLI tool: used to create, call, and manage `jsmv` contracts and accounts.

### Testing ✅

Unit and integration tests can be run using:

```sh
make test
```

To run `jsmv_kernel` in debug mode, the `mavkit-smart-rollup-wasm-debugger` should be used.

```sh
make build
./mavkit-smart-rollup-wasm-debugger \
    --kernel ./target/wasm32-unknown-unknown/release/jsmv_kernel.wasm \
    --inputs ./inputs.json
```

Once the REPL loads, to populate the rollup inbox, run:

```sh
> load inputs
```

To run the kernel with the inputs:

```sh
> step inbox
```

### Documentation 📚

#### Runtime API documentation

To edit documentation:

- Find or add a documentation file in `docs/api/`
- Modify documentation in markdown
- Locally test the documentation (with live reload) using
  ```sh
  npm run docs:dev
  ```

## Creating a pull request 📩

Please write a meaningful description for your pull request. If your pull request references an issue or Asana task, please mention it in the description. The format for pull request titles is `component/kind: description`.

For 'stacked' pull requests, please ensure your pull request lists all dependencies and uses the predecessor branch as the target.

To make sure your pull request is easy to review:

- **Use `git rebase`**. We maintain a semi-linear git history. This means that your branch should be a direct suffix of `main` (or the target branch). Addtionally, it should not contain any merge commits.
- **Don't push fixup commits\***. When your reviewer asks for changes, they will want you to rewrite your branch history so that the commit history is clean.

  If you branch history is dirty (containing fixup commits, etc) then we will squash-merge\*. However, this is undesirable as we lose the information that individual commits provide.

- **Follow the Rust style guide**. Please see the [Rust style guide](https://doc.rust-lang.org/nightly/style-guide/). Additionally ensure your code is formatted using

  ```sh
  make fmt
  ```

  Consider installing our pre-commit hook using

  ```sh
  ./scripts/install-pre-commit-hook.sh
  ```

- **Document your code**. Write documentation for your changes, either as comments or as a markdown file in `/docs`.

- **Do not submit untested code**. If you are not able to build or run `jsmv` locally to verify that your changes work as expected, please do not submit the changes -- unless the PR is marked as a 'draft'.
