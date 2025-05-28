# Special place where only the data files for the holo and gamescope portals
# will be installed (holo.portal, gamescope.portal and gamescope-portals.conf).
# The /usr/bin/start-gamescope-session script makes the
# $XDG_DESKTOP_PORTAL_DIR environment variable point to that place to ensure
# that no other portals are found.
portal_dir = datadir / 'xdg-desktop-portal' / 'gamescope'

service_file = 'xdg-desktop-portal-gamescope.service'

data_conf = configuration_data()
data_conf.set('busname', busname)
data_conf.set('portalbin', libexecdir / binary)
data_conf.set('servicefile', service_file)

fs = import('fs')

systemd = dependency('systemd')
systemd_default_prefix = systemd.get_variable(pkgconfig: 'prefix')
systemd_user_unit_dir = systemd.get_variable(pkgconfig: 'systemd_user_unit_dir')
systemd_user_unit_dir = prefix / fs.relative_to(systemd_user_unit_dir, systemd_default_prefix)

dbus = dependency('dbus-1')
dbus_default_prefix = dbus.get_variable(pkgconfig: 'prefix')
session_bus_services_dir = dbus.get_variable(pkgconfig: 'session_bus_services_dir')
session_bus_services_dir = prefix / fs.relative_to(session_bus_services_dir, dbus_default_prefix)

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
