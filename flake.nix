{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = inputs:
    with inputs;
      flake-utils.lib.eachDefaultSystem (
        system: let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [(import rust-overlay)];
          };

          makeFrameworkFlags = frameworks:
            pkgs.lib.concatStringsSep " " (
              pkgs.lib.concatMap
              (
                framework: [
                  "-F${pkgs.darwin.apple_sdk.frameworks.${framework}}/Library/Frameworks"
                  "-framework ${framework}"
                ]
              )
              frameworks
            );

          clangNoArch =
            if pkgs.stdenv.isDarwin
            then
              pkgs.clang.overrideAttrs (old: {
                postFixup = ''
                  ${old.postFixup or ""}

                  # On macOS this contains '-march' and '-mcpu' flags. These flags
                  # would be used for any invocation of Clang.
                  # Removing those makes the resulting Clang wrapper usable when
                  # cross-compiling where passing '-march' and '-mcpu' would not
                  # make sense.
                  echo > $out/nix-support/cc-cflags-before
                '';
              })
            else pkgs.clang;

          jsmv = pkgs.callPackage ./nix/jsmv.nix {makeFrameworkFlags = makeFrameworkFlags;};
        in {
          packages = {
            inherit (jsmv) jsmv_core jsmv_api jsmv_crypto jsmv_proto jsmv_kernel jsmv_cli js_jsmv js_jsmv-types;
            default = jsmv.jsmv_kernel;
          };

          # Rust dev environment
          devShells.default = pkgs.mkShell {
            NIX_LDFLAGS = pkgs.lib.optional pkgs.stdenv.isDarwin (
              makeFrameworkFlags [
                "Security"
                "SystemConfiguration"
              ]
            );

            CC = "clang";

            # This tells the 'cc' Rust crate to build using this C compiler when
            # targeting other architectures.
            CC_wasm32_unknown_unknown = "${clangNoArch}/bin/clang";
            CC_riscv64gc_unknown_hermit = "${clangNoArch}/bin/clang";

            hardeningDisable =
              pkgs.lib.optionals
              (pkgs.stdenv.isAarch64 && pkgs.stdenv.isDarwin)
              ["stackprotector"];

            shellHook = with pkgs;
              lib.strings.concatLines
              ([
                  ''
                    npm install
                    export PATH="$PWD/node_modules/.bin/:$PATH"
                  ''
                ]
                ++ lib.optionals stdenv.isLinux [
                  ''
                    export PKG_CONFIG_PATH=${openssl.dev}/lib/pkgconfig
                  ''
                ]);

            buildInputs = with pkgs;
              [
                llvmPackages_16.clangNoLibc
                (rust-bin.stable."1.73.0".default.override {
                  targets = ["wasm32-unknown-unknown"];
                })
                rust-analyzer
                wabt

                nodejs
                prefetch-npm-deps

                alejandra

                python311Packages.base58
                jq
              ]
              ++ lib.optionals stdenv.isLinux [pkg-config openssl.dev];
          };
        }
      );
}
