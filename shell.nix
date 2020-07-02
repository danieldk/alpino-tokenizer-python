with import <nixpkgs> {};

mkShell {
  nativeBuildInputs = [
    cargo
    clippy
    maturin
  ];

  buildInputs = [
    python3
  ];
}
