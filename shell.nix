with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "building-environment";
  buildInputs = [ pkg-config imagemagick lato ];

  LIBCLANG_PATH =
    pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
}
