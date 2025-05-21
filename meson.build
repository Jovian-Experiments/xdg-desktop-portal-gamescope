project(
  'xdg-desktop-portal-gamescope', 'rust',
  version: '0.1',
  license: 'LGPL-2.1-or-later',
  meson_version: '>=1.3',
)

cargo = find_program('cargo', required: true)
cargo_version = run_command(cargo, '--version', check: true).stdout().strip()
message(cargo_version)
rustc_version = run_command('rustc', '--version', check: true).stdout().strip()
message(rustc_version)

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
datadir = prefix / get_option('datadir')
libdir = prefix / get_option('libdir')

busname = 'org.freedesktop.impl.portal.desktop.gamescope'
binary = 'xdg-desktop-portal-gamescope'

subdir('data')
subdir('src')
