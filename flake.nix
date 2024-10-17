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
      ./assets
      ./src
      ./Cargo.lock
      ./Cargo.toml
    ];

    wasmFiles = fs.unions [
      ./www
      files
    ];
    wasmSrc = fs.toSource {
      root = ./.;
      fileset = wasmFiles;
    };
    craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

    wasmInputs = with pkgs; [
      lld
      wasm-bindgen-cli
    ];

    webArtifacts = craneLib.buildDepsOnly {
      src = wasmSrc;
      buildInputs = wasmInputs;
      # version = "0.1.0";

      cargoExtraArgs = "--target wasm32-unknown-unknown";
      doCheck = false;
    };
    devArtifacts = craneLib.buildDepsOnly {
      src = wasmSrc;
      CARGO_PROFILE = "dev";
      buildInputs = wasmInputs;
      # version = "0.1.0";

      cargoExtraArgs = "--target wasm32-unknown-unknown";
      doCheck = false;
    };
    webSite = craneLib.buildPackage {
      src = wasmSrc;
      cargoArtifacts = webArtifacts;
      buildInputs = wasmInputs;
      cargoExtraArgs = "--target wasm32-unknown-unknown";
      doCheck = false;
      postFixup = ''
        # mkdir $out/bin/wasm
        # cp -r assets $out/bin/wasm/

        cd $out/bin
        cp -r ${./www}/* .

        wasm-bindgen --no-typescript --target web \
          --out-dir . \
          ${devSite.pname}.wasm
      '';
    };
    devSite = craneLib.buildPackage {
      src = wasmSrc;
      CARGO_PROFILE = "dev";
      cargoArtifacts = devArtifacts;
      buildInputs = wasmInputs;
      cargoExtraArgs = "--target wasm32-unknown-unknown";
      doCheck = false;
      postFixup = ''
        # mkdir $out/bin/wasm
        # cp -r assets $out/bin/wasm/

        cd $out/bin
        cp -r ${./www}/* .

        wasm-bindgen --no-typescript --target web \
          --out-dir . \
          ${devSite.pname}.wasm
      '';
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
    devServe = pkgs.writeScriptBin "devrun" ''
      car build --target wasm32-unknown-unknown
      cp target/wasm32-unknown-unknown/debug/rustgl.wasm tmp
      wasm-bindgen --no-typescript --target web --out-dir tmp tmp/rustgl.wasm
      static-server tmp
    '';
    nixServe = pkgs.writeScriptBin "serve" ''
      ${pkgs.static-server}/bin/static-server ${webSite}/bin/
    '';
  in {
    packages.${system} = {
      hello = pkgs.hello;
      default = self.packages.x86_64-linux.hello;
      wasmDeps = devArtifacts;
      devSite = devSite;
      web = webSite;
      serve = nixServe;
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
        devServe
      ];
    };
  };
}
