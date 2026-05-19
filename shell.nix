{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  # 개발 환경에 포함할 패키지 목록
  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    clippy
    rust-analyzer # IDE 지원을 위한 언어 서버
  ];

  # 필요한 경우 환경 변수 설정
  shellHook = ''
    export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
    echo "🦀 Welcome to the Rust development environment! 🦀"
    rustc --version
  '';
}
