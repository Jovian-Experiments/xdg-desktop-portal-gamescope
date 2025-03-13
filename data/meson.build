portal_dir = datadir / 'xdg-desktop-portal' / 'gamescope'
service_file = 'xdg-desktop-portal-gamescope.service'

data_conf = configuration_data()
data_conf.set('busname', busname)
data_conf.set('portalbin', bindir / binary)
data_conf.set('servicefile', service_file)

configure_file(
  input: 'gamescope.portal.in',
  output: '@BASENAME@',
  configuration: data_conf,
  install: true,
  install_dir: portal_dir,
)

configure_file(
  input: busname + '.service.in',
  output: '@BASENAME@',
  configuration: data_conf,
  install: true,
  install_dir: datadir / 'dbus-1' / 'services',
)

configure_file(
  input: binary + '.desktop.in',
  output: '@BASENAME@',
  configuration: data_conf,
  install: true,
  install_dir: datadir / 'applications',
)

configure_file(
  input: binary + '.service.in',
  output: '@BASENAME@',
  configuration: data_conf,
  install: true,
  install_dir: libdir / 'systemd' / 'user',
)