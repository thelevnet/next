{
  description = "Next - NixOS CLI wrapper";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          runtimeDeps = with pkgs; [
            nh
            fzf
            jq
            git
            nix
          ] ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux [
            pkgs.nixos-rebuild
          ];
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "next";
            version = "1.1.4";
            src = ./.;

            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = [
              pkgs.installShellFiles
              pkgs.makeWrapper
            ];

            postInstall = ''
              installShellCompletion --cmd next \
                --zsh <($out/bin/next completions zsh) \
                --bash <($out/bin/next completions bash)

              wrapProgram $out/bin/next \
                --prefix PATH : ${pkgs.lib.makeBinPath runtimeDeps}
            '';

            meta = with pkgs.lib; {
              description = "NixOS CLI wrapper";
              mainProgram = "next";
            };
          };
        }
      );

      apps = forAllSystems (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/next";
        };
      });

      overlays.default = final: prev: {
        next = self.packages.${prev.stdenv.hostPlatform.system}.default;
      };

      devShells = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${system}.default ];
            packages = with pkgs; [ cargo rustc rust-analyzer ];
          };
        }
      );
    };
}
