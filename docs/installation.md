# 📦 Installing `jsmv`

Currently, `jsmv` can only be installed by building from sources using [Rust](https://www.rust-lang.org/).

## Download and Install

::: danger
⚠️ Under construction ⚠️
:::

## Building from Source

Below are instruction on how to build `jsmv` from source. Additionally, this section
details how to install Mavkit, which is used for our local sandbox.

### Cloning the Repository

```sh
git clone https://github.com/mavryk-network/jsmv.git
```

### Prerequisites 📋

::: tip  
Both `jsmv` and Mavkit are packaged with Nix, a package manager and system configuration tool that makes building from sources easy! See the [Nix docs](https://nixos.org/download.html) for instructions for your system. Additionally, ensure [Nix flakes are enabled](https://nixos.wiki/wiki/Flakes#Enable_flakes).
:::

#### Rust 🦀

> `jsmv` requires a specific release of Rust. The version of Rust required is specified in the `rust-toolchain` file.

Install the [Rust](https://rustup.rs/) toolchain:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once `rustup` is installed, the build dependencies can be installed with:

```sh
make build-deps
```

::: tip
Using Nix, simply run `nix develop` to enter a shell with all build dependencies
:::

#### Mavkit 🐙

Our sandbox network uses a custom distribution of Mavkit found [here](https://gitlab.com/mavryk-network/mavryk-protocol/-/tree/6c0621760ddce94afeff3484d9e8a650d8535f25). See the [Mavkit docs](https://protocol.mavryk.org/introduction/howtoget.html?highlight=building#compiling-with-make) for instructions on building Mavkit from source.

Once Mavkit has been built, copy the following binaries to `jsmv`:

- `mavkit-client`
- `mavkit-node`
- `mavkit-smart-rollup-node`
- `mavkit-smart-rollup-wasm-debugger`

::: tip

Using Nix, simply execute the following:

```sh
# Clone Mavkit
git clone git@gitlab.com:mavryk-network/mavryk-protocol.git
cd mavryk
# Checkout custom distribution
git checkout ole@next-gen@floats
# Build using Nix
nix-build -j auto
```

After Nix successfully builds Mavkit (it takes a long time 🕣), the Mavkit binaries will be accessable from `result`.
:::

### Building 👷‍♂️

The `.wasm` file for `jsmv`'s kernel is built with:

```sh
make build
```

You can locate the resulting build artifact at `/target/wasm32-unknown-unknown/release/jsmv_kernel.wasm`.

### Running the Sandbox 🏝️

You can now start the sandbox with:

```sh
make build-installer
cargo run -- sandbox start
```

This will initially run `mavkit-node` and initialize `mavkit-client`. Once the client is initialized, the `jsmv_kernel` and `jsmv_bridge` is originated and a smart-rollup node is spun up.
