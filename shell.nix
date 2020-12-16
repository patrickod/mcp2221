let
  rust-version = "stable";

  nixpkgs = fetchGit {
    url = "https://github.com/patrickod/nixpkgs.git";
    rev = "035a4986bc72475b007277d3f80195a935ea1a55";
    ref = "personal";
  };

  mozilla-overlay =
    import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);

  pkgs = import nixpkgs {
    overlays = [ mozilla-overlay ];
  };

in
  pkgs.mkShell {
    name = "rust-dev";
    buildInputs = with pkgs; [
      (rustChannels.stable.rust.override {
        extensions = [
          "rust-src"
          "rust-analysis"
          "rls-preview"
        ];
      })
      cargo
      libusb1
      pkg-config
    ];
  }
