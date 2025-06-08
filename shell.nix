{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    rust-analyzer
    cargo-tauri
    nodejs_20
    webkitgtk_4_1
    librsvg
    glib
    atk
    pango
    cairo
    gtk3
    pkg-config
    libsoup_3

    # Use the dev packages explicitly
    glib.dev
    gdk-pixbuf.dev
    atk.dev
    pango.dev
    cairo.dev
    gtk3.dev
    webkitgtk_4_1.dev
  ];

  shellHook = ''
    export PKG_CONFIG_PATH=${pkgs.glib.dev}/lib/pkgconfig:${pkgs.gdk-pixbuf.dev}/lib/pkgconfig:${pkgs.atk.dev}/lib/pkgconfig:${pkgs.pango.dev}/lib/pkgconfig:${pkgs.cairo.dev}/lib/pkgconfig:${pkgs.gtk3.dev}/lib/pkgconfig
  '';
}
