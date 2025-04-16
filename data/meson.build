# Special place where only the data files for the holo and gamescope portals
# will be installed (holo.portal, gamescope.portal and gamescope-portals.conf).
# The /usr/bin/start-gamescope-session script makes the
# $XDG_DESKTOP_PORTAL_DIR environment variable point to that place to ensure
# that no other portals are found.
portal_dir = datadir / 'xdg-desktop-portal' / 'gamescope'

service_file = 'xdg-desktop-portal-gamescope.service'

data_conf = configuration_data()
data_conf.set('busname', busname)
data_conf.set('portalbin', bindir / binary)
data_conf.set('servicefile', service_file)

systemd = dependency('systemd')
systemd_user_unit_dir = systemd.get_variable(pkgconfig: 'systemd_user_unit_dir')

dbus = dependency('dbus-1')
session_bus_services_dir = dbus.get_variable(pkgconfig: 'session_bus_services_dir')

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
  install_dir: session_bus_services_dir,
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
  install_dir: systemd_user_unit_dir,
)
