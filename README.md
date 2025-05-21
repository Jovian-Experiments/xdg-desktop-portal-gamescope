# XDG desktop portal for gamescope-specific interfaces

This XDG desktop portal backend implements the following [backend interfaces](https://flatpak.github.io/xdg-desktop-portal/docs/impl-dbus-interfaces.html):

* [Access](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.impl.portal.Access.html)
* [ScreenCast](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.impl.portal.ScreenCast.html)
* [Screenshot](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.impl.portal.Screenshot.html)

## How to build and install

```shell
$ meson setup _build .
$ ninja -C _build
$ cd _build
$ meson install
```

## Authors

Olivier Tilloy <otilloy@igalia.com>

## License

xdg-desktop-portal-gamescope is published under the [3-Clause BSD license](LICENSE).
