{
  pkgs,
  makeRustPlatform,
  buildNpmPackage,
  makeFrameworkFlags,
}: let
  # TODO: read this from the rust-toolchain file
  rustVersion = "1.73.0";

  wasmTarget = "wasm32-unknown-unknown";

  rustWithWasmTarget = pkgs.rust-bin.stable.${rustVersion}.default.override {
    targets = [wasmTarget];
  };

  rustPlatformWasm = makeRustPlatform {
    cargo = rustWithWasmTarget;
    rustc = rustWithWasmTarget;
    llvmPackages = pkgs.llvmPackages_16;
  };

  common = {
    version = "0.1.0";
    src = ../.;

    # Needed to get openssl-sys (required by `jsmv_proto`) to use pkg-config.
    nativeBuildInputs = with pkgs; lib.optionals stdenv.isLinux [pkg-config];

    # Needed to get openssl-sys to use pkg-config.
    # Doesn't seem to like OpenSSL 3
    OPENSSL_NO_VENDOR = 1;

    buildInputs = with pkgs; lib.optionals stdenv.isLinux [openssl openssl.dev];

    NIX_LDFLAGS = pkgs.lib.optional pkgs.stdenv.isDarwin (
      makeFrameworkFlags [
        "Security"
        "SystemConfiguration"
      ]
    );

    cargoLock = {
      lockFile = ../Cargo.lock;
      outputHashes = {
        "mavryk-smart-rollup-0.2.2" = "sha256-v0ayPeHzhGzCdaHLpYh0bQm1569KrHgR/IxCXwBwhQU=";
        "boa_engine-0.17.0" = "sha256-RNFuFvBwDA/tjhEonFnn3t1q5DDehQkqLteJpWIXTFU=";
        "boa_gc-0.17.0" = "sha256-bf6i5ESIHwepb1a4dUYREPprz7Rijq+P5z+NXpsT16Q=";
        "hermit-0.7.2" = "sha256-GJLujJml6IpT1+rbOG0BdDVkoI1PQGc3McryTggPu+o=";
      };
    };
  };

  crate = pname:
    pkgs.rustPlatform.buildRustPackage (common
      // {
        pname = pname;
        cargoBuildFlags = "-p ${pname}";
      });

  kernel = pname:
    rustPlatformWasm.buildRustPackage (common
      // {
        pname = pname;

        NIX_CFLAGS_COMPILE = "-mcpu=generic";
        nativeBuildInputs = [pkgs.llvmPackages_16.clangNoLibc];
        hardeningDisable =
          pkgs.lib.optionals
          (pkgs.stdenv.isAarch64 && pkgs.stdenv.isDarwin)
          ["stackprotector"];

        # Don't run the tests (this runs all tests for the workspace which we want to do
        # at a later point)
        doCheck = false;

        buildPhase = ''
          CC=clang cargo build --release -p ${pname} --target=wasm32-unknown-unknown
        '';

        installPhase = ''
          mkdir -p $out/lib
          cp target/wasm32-unknown-unknown/release/*.wasm $out/lib/
        '';
      });

  jsPackage = pname:
    buildNpmPackage {
      name = pname;
      src = ../packages/${pname};
      npmDepsHash = "sha256-gHkv831Mknd7McNJzzvIf7s5gwdErdHtMti8nkZGBjk=";
    };
in {
  jsmv_core = crate "jsmv_core";

  jsmv_api = crate "jsmv_api";

  jsmv_crypto = crate "jsmv_crypto";

  jsmv_proto = crate "jsmv_proto";

  jsmv_kernel = kernel "jsmv_kernel";

  jsmv_cli = crate "jsmv_cli";

  jsmv_node = crate "jsmv_node";

  js_jsmv = jsPackage "jsmv";

  js_jsmv-types = jsPackage "jsmv-types";
}