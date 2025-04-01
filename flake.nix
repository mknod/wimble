{
  description = "My Rust Project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-devshell.url = "github:mknod/rust-devshell";
    rust-devshell.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-devshell, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        config.allowUnfree = true; # Needed for Chrome
      };
      rustShell = rust-devshell.devShells.${system}.default.overrideAttrs (old: {
        buildInputs = old.buildInputs ++ [ pkgs.google-chrome pkgs.chromedriver ];
        shellHook = ''
        mkdir -p "$HOME/.cache/nix-chrome"
        ln -sf ${pkgs.google-chrome}/bin/google-chrome-stable "$HOME/.cache/nix-chrome/google-chrome"
        export PATH="$HOME/.cache/nix-chrome:$PATH"

        echo "✅ Aliased google-chrome-stable → google-chrome"
        echo "🔍 which google-chrome: $(which google-chrome)"
        '';
      });
    in
    {
      devShells.${system}.default = rustShell;

      packages.${system}.default = rust-devshell.packages.${system}.rustc;
    };
}
