# Copyright © 2025 Valve Corporation
#
# SPDX-License-Identifier: BSD-3-Clause

project(
  'xdg-desktop-portal-gamescope', 'rust',
  version: '0.1',
  license: 'BSD-3-Clause',
  meson_version: '>=1.3',
)

cargo = find_program('cargo', required: true)
cargo_version = run_command(cargo, '--version', check: true).stdout().strip()
message(cargo_version)
rustc_version = run_command('rustc', '--version', check: true).stdout().strip()
message(rustc_version)

prefix = get_option('prefix')
datadir = prefix / get_option('datadir')
libexecdir = prefix / get_option('libexecdir')

busname = 'org.freedesktop.impl.portal.desktop.gamescope'
binary = meson.project_name()

subdir('data')
subdir('src')
