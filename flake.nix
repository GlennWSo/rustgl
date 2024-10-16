{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    crane,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [fenix.overlays.default];
    };
    toolchain = pkgs.fenix.combine [
      pkgs.cargo
      pkgs.rustc
      pkgs.rust-analyzer
      pkgs.fenix.targets.wasm32-unknown-unknown.latest.rust-std
    ];
    fs = pkgs.lib.fileset;
    files = fs.unions [
      # ./.cargo
      # ./assets
      ./src
      ./www
      ./Cargo.lock
      ./Cargo.toml
    ];
    src = fs.toSource {
      root = ./.;
      fileset = files;
    };
    craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

    wasmInputs = with pkgs; [
      lld
      wasm-bindgen-cli
    ];

    wasmArtifacts = craneLib.buildDepsOnly {
      inherit src;
      buildInputs = wasmInputs;
      version = "0.1.0";

      cargoExtraArgs = "--target wasm32-unknown-unknown";
      doCheck = false;
    };

    nixRuntime = with pkgs; [
      wayland
      libxkbcommon
      udev.dev
      vulkan-loader

      pkg-config
      alsaLib
      alsaLib.dev
      libGL
      glib
      gtk3
    ];
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nixRuntime;
    car = pkgs.writeScriptBin "car" ''
      LD_LIBRARY_PATH=${LD_LIBRARY_PATH} cargo $@
    '';
  in {
    packages.${system} = {
      hello = pkgs.hello;
      default = self.packages.x86_64-linux.hello;
      wasmDeps = wasmArtifacts;
    };
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        toolchain
        rustfmt
        glsl_analyzer
        wasm-bindgen-cli
        lld

        glslviewer
        car
        static-server
      ];
    };
  };
}
