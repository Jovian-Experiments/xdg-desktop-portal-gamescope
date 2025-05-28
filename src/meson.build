# Copyright © 2025 Valve Corporation
#
# SPDX-License-Identifier: BSD-3-Clause

src_conf = configuration_data()
src_conf.set('busname', busname)
configure_file(
  input: 'config.rs.in',
  output: '@BASENAME@',
  configuration: src_conf
)


cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]

if get_option('profile') == 'development'
  rust_target = 'debug'
  message('Building in debug mode')
else
  cargo_options += [ '--release' ]
  rust_target = 'release'
  message('Building in release mode')
endif

cargo_env = [ 'CARGO_HOME=' + meson.project_build_root() / 'cargo-home' ]
cargo_env += [ 'CARGO_TARGET_DIR=' + meson.project_build_root() / 'src' ]

custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: binary,
  console: true,
  install: true,
  install_dir: libexecdir,
  command: [
    'env',
    cargo_env,
    cargo, 'build',
    cargo_options,
    '&&',
    'cp', 'src' / rust_target / binary, '@OUTPUT@',
  ]
)
