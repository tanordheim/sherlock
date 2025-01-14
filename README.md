# Sherlock Application Launcher
<div align="center" style="text-align:center;">
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


## Features
### 🔧 Style Customization
- Fully customize the look and feel of the launcher.
- Modify themes and visual elements to match your preferences

### 🛠️ Custom Commands
- Define your own commands and extend the functionality of the launcher.
- Add new feartures or workflows tailored to your specifig needs.

### 🚀 Fallbacks
- Configure fallback behaviours for various commands and operations.
- Ensure a smooth experience even when certain commands fail or are unavailable.

### 🖼️ Application Aliases and Customization
- Assign aliases to your applications for better looks and quicker access.
- Assign cusom icons to your applications for a personalized touch.
- Hide apps that you don't use and don't want to clutter up your launcher.

### 🌐 Async Widget
- Use the async widget to send API requests and display their responses directly in the launcher.
- Great for integrating live data or external tools into your workflow.

### 🔍 Category-Based Search 
- Type the launcher alias and spacebar to only search within a specific category of commands.
- Categories are fully configureable, allowing you to customize search scopes.



## Getting Started
## Dependencies
- gtk4
- gtk-4-layer-shell

### InstallationS
#### Arch Linux
```bash
yay -S sherlock-launcher-bin
```

#### From Source
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
