# Sherlock Application Launcher
<div align="center" style="text-align:center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="images/logo-light.svg">
    <source media="(prefers-color-scheme: light)" srcset="images/logo-light.svg">
    <img alt="sherlock logo" height="250" src="images/logo-light.svg">
  </picture>
</div>


## Dependencies
- gtk4
- gtk-4-layer-shell

# Installation Guide for Sherlock

## Introduction
Sherlock is a lightweight and efficient application launcher built with Rust and GTK4. It allows you to quickly launch your favorite applications with a user-friendly interface, providing a fast and highly-configurable way to search, launch, and track application usage.

> **Warning:** The app is was created on Arch Linux with the Hyprland tiling window manager in mind. It may cause errors or won't function at all on other system configurations.

## Prerequisites
Before you begin, make sure you have the following installed:
- [Rust](https://www.rust-lang.org/) (>= 1.50.0)
- [Git](https://git-scm.com/)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)
- gtk4
- gtk-4-layer-shell

## Cloning the repo
```bash 
git clone https:/&github.com/Skxxtz/sherlock.git
cd sherlock
```

## Building the app
To build the application, run the `cargo build` command.
```bash 
cargo build --release
sudo cp target/release/sherlock /usr/bin/
```

## Running the app
To now start the app type `sherlock`. If started from the terminal, it will send debug messages to that terminal. However, its recommended to bind the app to a keybind for easier access.

