{
  description = "hachiware — Rust development environment";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-26.05";
    flake-utils.url = "github:numtide/flake-utils";

    # Rust toolchains from the official rust-lang release channels, packaged
    # for Nix. Preferred over nixpkgs' own `rustc`/`cargo` because it pins an
    # exact upstream release and can add cross targets and nightly components
    # without a nixpkgs bump.
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachSystem [
      flake-utils.lib.system.aarch64-darwin
      flake-utils.lib.system.x86_64-darwin
      # Kept so CI runners (and any Linux box) resolve the same shell.
      flake-utils.lib.system.x86_64-linux
      flake-utils.lib.system.aarch64-linux
    ] (system:
      let
        # The Rust version is pinned HERE, not in flake.lock: `toolchainOf`
        # fetches the rust-lang channel manifest directly, so bumping Rust is a
        # two-line edit (version + hash) that `nix flake update` cannot drift.
        # The hash covers the platform-independent channel manifest, so one
        # value is correct for every system listed above.
        #
        # To bump: change `rustVersion`, set `rustSha` to
        # lib.fakeSha256 ("sha256-AAAA...="), run `nix develop`, and paste the
        # hash Nix reports as the correct one.
        rustVersion = "1.98.1";
        rustSha = "sha256-p8h3Sl/YRByZfZTAKXdsvF6xEenXKrXSVvpphmZENH4=";

        pkgs = import nixpkgs { inherit system; };

        # completeToolchain = rustc, cargo, rustfmt, clippy, rust-src,
        # rust-analyzer and friends as one derivation, all at the same version.
        # Add cross targets later with `fenix.packages.${system}.combine`
        # (see symmatrics' flake.nix for the wasm/musl pattern).
        rustToolchain = (fenix.packages.${system}.toolchainOf {
          channel = rustVersion;
          sha256 = rustSha;
        }).completeToolchain;
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [ rustToolchain ] ++ (with pkgs; [
            # Build-time helpers most -sys crates expect to find on PATH.
            pkg-config

            cargo-nextest
            cargo-deny
            cargo-watch

            git
          ]);

          env = {
            RUST_BACKTRACE = "1";
            # rust-analyzer needs an explicit pointer at the std sources,
            # since fenix ships them in the toolchain's own store path.
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          };

          shellHook = ''
            echo "hachiware dev shell — $(rustc --version)"
          '';
        };

        formatter = pkgs.nixpkgs-fmt;
      });
}
