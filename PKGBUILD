# Maintainer: Sebastian Kootz - skxxtz@gmail.com
pkgname=sherlock
pkgver=0.1.0
pkgrel=1
pkgdesc="Application launcher for Wayland."
arch=('x86_64')
url="https://github.com/skxxtz/sherlock"
license=('custom:CC-BY-NC-4.0')
depends=('gtk4' 'gtk4-layer-shell')
makedepends=('cargo' 'rust')
source=("https://github.com/Skxxtz/sherlock/releases/download/v${pkgver}/sherlock-v${pkgver}-linux-x86_64.tar.gz")
sha256sums=('SKIP')


build() {
    cd "$srcdir"
    cargo build --release
}

package(){
    cd "$srcdir"
    install -Dm755 target/release/sherlock "$pkgdir/usr/bin/sherlock"
    install -Dm644 "$srcdir/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
