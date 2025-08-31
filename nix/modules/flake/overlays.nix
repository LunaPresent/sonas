{ self, ... }:
{

  flake.overlays.default =
    final: prev:
    let
      inherit (prev.stdenv.hostPlatform) system;
    in
    if builtins.hasAttr system self.packages then
      { sonas = self.packages.${system}.default; }
    else
      { };
}
