# Sherlock Application Launcher
<div align="center" style="text-align:center; margin-bottom:10em;">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="images/logo-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="images/logo-light.svg">
    <img alt="sherlock logo" height="250" src="images/logo-light.svg">
  </picture>
  <picture>
    <img alt="application screenshot" width="100%" src="images/home-screen.png">
  </picture>
</div>

Sherlock is a lightweight and efficient application launcher built with Rust and GTK4. It allows you to quickly launch your favorite applications with a user-friendly interface, providing a fast and highly-configurable way to search, launch, and track application usage.

> **Warning:** The app is was created on Arch Linux with the Hyprland tiling window manager in mind. It may cause errors or won't function at all on other system configurations.


## Dependencies
- gtk4
- gtk-4-layer-shell

## InstallationS
### Arch Linux
```bash
yay -S sherlock-launcher-bin
```

### From Source
Make sure you have the following dependencies installed:
- Rust
- Cargo
- gtk4
- gtk-4-layer-shell

```bash
git clone https://github.com/skxxtz/sherlock.git
cd sherlock
cargo build --release
sudo cp target/release/sherlock /usr/bin/
```
