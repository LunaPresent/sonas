{
  perSystem =
    { pkgs, self', ... }:
    {
      devShells.default = pkgs.mkShell {
        name = "sonas";
        meta.description = "Development environment for sonas";
        RUST_SRC_PATH = builtins.toString pkgs.rust.packages.stable.rustPlatform.rustLibSrc;

        nativeBuildInputs = [
          pkgs.cargo
          pkgs.pkg-config
          pkgs.rustc
          self'.formatter
        ];
      };
    };
}
