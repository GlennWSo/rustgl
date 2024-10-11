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
  in {
    packages.${system} = {
      hello = pkgs.hello;
      default = self.packages.x86_64-linux.hello;
    };
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = [
        toolchain
      ];
    };
  };
}
