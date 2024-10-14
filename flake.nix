{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
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
    ];
    nativeRuntime = with pkgs; [
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
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeRuntime;
    car = pkgs.writeScriptBin "car" ''
      LD_LIBRARY_PATH=${LD_LIBRARY_PATH} cargo $@
    '';
  in {
    packages.${system} = {
      hello = pkgs.hello;
      default = self.packages.x86_64-linux.hello;
    };
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        toolchain
        rustfmt
        car
      ];
    };
  };
}
