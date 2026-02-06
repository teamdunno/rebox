{...}: {
  perSystem = {pkgs, ...}: {
    devShells.default = pkgs.mkShell {
      packages = [pkgs.cargo];
    };
  };
}
