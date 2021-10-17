{ pkgs }: {
    deps = [
        pkgs.cargo
        pkgs.cargo-edit
        pkgs.rustc
        pkgs.rustfmt
        pkgs.rustup
    ];
}
