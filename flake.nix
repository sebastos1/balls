{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-22.11";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {inherit system overlays;};
      rust = (pkgs.rust-bin.nightly.latest.default.override {
        extensions = ["rust-src"];
        targets = ["wasm32-unknown-unknown"];
      });
      platform = pkgs.makeRustPlatform {
        cargo = rust;
        rustc = rust;
      };
      shellInputs = with pkgs; [
        clang
        trunk
        aseprite
        (rust.override {extensions = ["rust-src"];})
      ];
      appNativeBuildInputs = with pkgs; [
        pkg-config
      ];
      appRuntimeInputs = with pkgs; [
        vulkan-loader
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
      ];
      appBuildInputs =
        appRuntimeInputs
        ++ (with pkgs; [
          udev
          mold
          alsaLib
          xlibsWrapper
          vulkan-tools
          vulkan-headers
          vulkan-validation-layers
        ]);
    in {
      defaultPackage = platform.buildRustPackage {
        src = ./.;
        pname = "balls";
        version = "0.1.0";
        cargoLock.lockFile = ./Cargo.lock;
        buildInputs = appBuildInputs;
        nativeBuildInputs = appNativeBuildInputs;
      };
      devShell = pkgs.mkShell {
        buildInputs = shellInputs ++ appBuildInputs;
        nativeBuildInputs = appNativeBuildInputs;
        shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath appBuildInputs}"'';
      };
    });
}