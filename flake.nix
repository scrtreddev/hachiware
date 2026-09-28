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

        # Raspberry Pi Pico 1 = RP2040 = Cortex-M0+ = `thumbv6m-none-eabi`.
        # (Pico 2 / RP2350 would be thumbv8m.main-none-eabihf instead — a
        # different target, not a newer version of this one.)
        picoTarget = "thumbv6m-none-eabi";

        # completeToolchain = rustc, cargo, rustfmt, clippy, rust-src,
        # rust-analyzer and friends, all at the same version. `combine` grafts
        # the cross-compiled `rust-std` for each extra target into that same
        # derivation, which is what makes `cargo build --target <t>` work
        # without rustup ever being involved.
        rustToolchain =
          let
            fx = fenix.packages.${system};
            base = fx.toolchainOf {
              channel = rustVersion;
              sha256 = rustSha;
            };
            # Same channel + hash as the host toolchain: the std for a cross
            # target must match the rustc that consumes it.
            picoStd = fx.targets.${picoTarget}.toolchainOf {
              channel = rustVersion;
              sha256 = rustSha;
            };
          in
          fx.combine [
            base.completeToolchain
            picoStd.rust-std
          ];
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [ rustToolchain ] ++ (with pkgs; [
            # Build-time helpers most -sys crates expect to find on PATH.
            pkg-config

            cargo-nextest
            cargo-deny
            cargo-watch

            # --- RP2040 flashing (no debug probe required) ---
            # Hold BOOTSEL while plugging the Pico in and it enumerates as a
            # USB mass-storage device. `elf2uf2-rs` turns cargo's ELF into the
            # .uf2 that volume accepts; `picotool load` pushes an ELF or .uf2
            # over USB directly, skipping the drag-and-drop.
            elf2uf2-rs
            picotool

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
