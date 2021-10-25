pkgname=gobble
pkgver=1.1
pkgrel=2
pkgdesc='Window swallowing on any WM'
url='https://github.com/EmperorPenguin18/gobble/'
source=("$pkgname-$pkgver.tar.gz::https://github.com/EmperorPenguin18/gobble/archive/refs/tags/$pkgver.tar.gz")
arch=('x86_64')
license=('GPL3')
makedepends=('cargo')
depends=('libxcb')
sha256sums=('a2fef6f612950cdc0595c39b7688bfb10300a10dfd4738d4e94ba979531dcc61')

build () {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release --target-dir target
}

package () {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 target/release/gobble -t "$pkgdir/usr/bin"
}
