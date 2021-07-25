#!/usr/bin/env bash

packagename="colstract"

build() {
    RUSTUP_TOOLCHAIN=stable cargo build --release --locked --all-features --target-dir=target
}

check() {
    RUSTUP_TOOLCHAIN=stable cargo test --locked --all-features --target-dir=target
}

create_tar() {
    mkdir -p "packaging_dir/usr/bin/${packagename}"
    mkdir -p "packaging_dir/usr/share/${packagename}"
    cp "target/release/${packagename}" "packaging_dir/usr/bin/${packagename}"
    cp -r "assets"/* "packaging_dir/usr/share/${packagename}"

    tar -czf "${packagename}.tar.gz" "packaging_dir"/*
}

check
build
create_tar
