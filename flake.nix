# flake.nix

{
  description = "flake file for eventrc dev environment";

  # ---------------------------------------------------------------------
  # options are bash-prompt, bash-prompt-prefix, and bash-prompt-suffix
  # commenting out -- see below for explanation [#PS1_PREFIX]
  # nixConfig.bash-prompt-prefix = "nix-dev";
  # ---------------------------------------------------------------------

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

	cargoBuildInputs = with pkgs; lib.optionals stdenv.isDarwin [                                                                                 darwin.apple_sdk.frameworks.CoreServices                                                                                                  ];

	rustBuildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            rustfmt
            clippy
          ] ++ cargoBuildInputs;

        nodePkgs = pkgs.nodePackages_latest;
        nodeBuildInputs = with nodePkgs; [
            nodejs
            npm
            pnpm
        ];
      in
        {
          devShell = pkgs.mkShell {
            # -----------------------------------------------------------
            # I would use the bash-prompt-prefix but I want it to work
            # with powerline-go ; in my .bashrc, I check for the PS1_PREFIXls
            # and when in a nix shell, I use that instead of "nix-shell"
            # -----------------------------------------------------------
            PS1_PREFIX = "nix dev";
            PS1_SUFFIX = "eventrc";

	    RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

            buildInputs = with pkgs; [
	    ] ++ rustBuildInputs ++ nodeBuildInputs;
          };

          packages = {
            default = pkgs.buildEnv {
              name = "eventrc-dev";
              ignoreCollisions = false;
              paths = [];
            };
          };
        }
    );
}
