{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        rustToolchain = (with inputs.fenix.packages.${system}; combine [
          latest.toolchain
          targets.wasm32-wasip2.latest.rust-std
        ]);
        rustPlatform = (
          pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          }
        );
      in
      rec {
        packages = rec {
            wasi-sdk = pkgs.stdenvNoCC.mkDerivation {
                name = "wasi-sdk";
                src = pkgs.fetchurl {
                    url = "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-24/wasi-sdk-24.0-x86_64-linux.tar.gz";
                    sha256 = "sha256-xsOKq1bl3oit9sHrycOujacviOwrZW+wJO2o1BZ6C8U=";
                };
                installPhase = ''
                    mkdir $out
                    tar -xf $src --strip-components=1 -C $out
                '';
                setupHook = pkgs.writeText "setup-hook.sh" ''
                  wasisdk() {
                    export WASI_SDK_PATH=@out@
                  }
                  postHooks+=(wasisdk)
                '';
            };
            wac-cli = pkgs.stdenvNoCC.mkDerivation {
                name = "wac-cli";
                src = pkgs.fetchurl {
                    url = "https://github.com/bytecodealliance/wac/releases/download/v0.6.1/wac-cli-x86_64-unknown-linux-musl";
                    sha256 = "sha256-or+myLu4kt8Mq44/5unHu1hd9fQs/ru2+kwxKpSWTjE=";
                };
                dontUnpack = true;
                installPhase = ''
                    mkdir -p $out/bin
                    cp $src $out/bin/wac
                    chmod +x $out/bin/wac
                '';
            };
            apps-native = let
              manifest = (pkgs.lib.importTOML (./apps/Cargo.toml)).package;
            in
            rustPlatform.buildRustPackage {
              pname = manifest.name;
              version = manifest.version;
              cargoLock.lockFile = ./Cargo.lock;
              src = pkgs.lib.cleanSource ./.;
              buildAndTestSubdir = [ "apps" ];
              nativeBuildInputs = with pkgs; [ cmake ];
            };
        };
        apps = {
          client-native = {
            type = "app";
            program = "${packages.apps-native}/bin/quiche-client";
          };
          server-native = {
            type = "app";
            program = "${packages.apps-native}/bin/quiche-server";
          };
        };
        devShells = rec {
          default = pkgs.mkShell.override {stdenv = pkgs.stdenvNoCC;} rec {
            buildInputs = with pkgs; [
              cmake
              curlHTTP3
              gcc
              rustToolchain
              wasm-tools
              wasmtime
              packages.wasi-sdk
              packages.wac-cli
            ];
          };
        };
      }
    );
}
