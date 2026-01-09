{
  description = "CLI utility for viewing system information, but with a twist";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "footfetch";
          version = "1.41.7";
          src = ./.;
          
          cargoHash = ""; 

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          meta = with pkgs.lib; {
            description = "CLI utility for viewing system information, but with a twist";
            homepage = "https://github.com/adenast/footfetch";
            license = licenses.mit;
            maintainers = [ "adenast" ];
          };
        };
      }
    );
}