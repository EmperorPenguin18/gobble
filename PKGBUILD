pkgname='gobble'
pkgver=1.0
pkgrel=1
pkgdesc='Window swallowing on any WM'
arch=('x86_64')
url='https://github.com/EmperorPenguin18/gobble/'
license=('GPL3')
depends=('libxcb')
makedepends=('rust')

build () {
  cd $startdir
  make
  mv target/release/gobble $srcdir/gobble
}

package () {
  install -Dm644 $startdir/LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  mkdir -p "$pkgdir/usr/bin"
  install -Dm755 $srcdir/gobble "$pkgdir/usr/bin/gobble"
}
