let
  shell = import ./shell.nix;
  pkgs = shell.pkgs;
  effectsSrc = builtins.fetchTarball
    "https://github.com/hercules-ci/hercules-ci-effects/archive/91fae5824f5f1199f61693c6590b4a89abaed9d7.tar.gz";
  inherit (import effectsSrc { inherit pkgs; }) effects;
in {
  inherit shell;
  build = effects.mkEffect {
    src = ./.;
    effectScript = ''
      cargo build
    '';
    inputs = [ shell.hook ];
  };
}
