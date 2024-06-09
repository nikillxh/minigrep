pkgname=('minigrep')
pkgrel='1'
pkgver='1.0'
pkgdesc="Tool for searching line of word"
arch=('x86_64')
makedepends=(cargo)
options=(debug !strip)
prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}
build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}
check() {
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}
package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}