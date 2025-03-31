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
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          rust-devshell.devShells.${system}.default
          pkgs.google-chrome
          pkgs.chromedriver
        ];

        # Optional: ensure ChromeDriver matches Chrome version (basic workaround)
        shellHook = ''
          echo "🦀 Rust dev shell with Chrome + ChromeDriver"
          echo "🔍 Google Chrome: $(google-chrome --version)"
          echo "🚗 ChromeDriver: $(chromedriver --version)"
        '';
      };

      packages.${system}.default = rust-devshell.packages.${system}.default;
    };
}
