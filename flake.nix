{
  description = "Wimble Devshell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    # Replace this with HTTPS if SSH isn't available on all environments
    rust-devshell.url = "git+ssh://git@cavern.rmrn.wtf:/rust-devshell.git";
    rust-devshell.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-devshell, ... }:
    rust-devshell.outputs // {
      # Optional: You can define your own devShell using rust-devshell tooling
      devShells.default = rust-devshell.devShells.default;
      packages.default = rust-devshell.packages.default;
    };
}
