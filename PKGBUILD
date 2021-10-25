pkgname='gobble'
pkgver=1.1
pkgrel=1
pkgdesc='Window swallowing on any WM'
arch=('x86_64')
url='https://github.com/EmperorPenguin18/gobble/'
license=('GPL3')
depends=('libxcb')
makedepends=('rust' 'pandoc' 'gzip')

build () {
  cd $startdir
  make
  mv target/release/gobble $srcdir/gobble
}

package () {
  install -Dm644 $startdir/LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  mkdir -p "$pkgdir/usr/bin"
  install -Dm755 $srcdir/gobble "$pkgdir/usr/bin/gobble"
  mkdir -p "$pkgdir/usr/share/man/man1"
  install -Dm644 $startdir/gobble.1.gz "$pkgdir/usr/share/man/man1/gobble.1.gz"
  mandb
}
