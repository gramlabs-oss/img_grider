with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "building-environment";
  buildInputs = [ pkg-config imagemagick ];

  LIBCLANG_PATH =
    pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
}
