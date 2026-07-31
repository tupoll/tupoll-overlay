# tupoll-overlay

The overlay is added with a simple Rust installation script and is ready to use. 
Add your ebuilds to it; it's local.

## Installation

To install **tupoll-overlay** for Gentoo, clone the repository:

```
git clone https://github.com/tupoll/tupoll-overlay.git

```
Install using the setup program (ensure you have root privileges):
```
cd $HOME/tupoll-overlay/files/tupoll-overlay/
cargo build --release
cd target/release/
sudo ./tupoll-overlay
cd
sudo mkdir -p /var/db/repos/tupoll-overlay/app-portage/
sudo mv -f $HOME/tupoll-overlay /var/db/repos/tupoll-overlay/app-portage/
sudo chown -R portage:portage /var/db/repos/tupoll-overlay
sudo ebuild /var/db/repos/tupoll-overlay/app-portage/tupoll-overlay/tupoll-overlay-9999.ebuild manifest
sudo eix-update
sudo emerge --ask app-portage/tupoll-overlay
sudo /usr/bin/tupoll-overlay

```

Clean up the installation directory $HOME 🗑:
```
rm -rf $HOME/tupoll-overlay

```

### 🛠️ Configuration
Before launching the WM, follow these steps:
1. Keywords Configuration
Add the following entries to /etc/portage/package.accept_keywords/:
```
media-video/soxbar **
app-portage/tupoll-overlay ~amd64 
gui-wm/pinnacle ~amd64
gui-wm/pinnacle-gentoo ~amd64
gui-apps/pinnacle-translator ~amd64
gui-apps/pinnacle-terminal ~amd64
gui-apps/pinnacle-wallpaper ~amd64
gui-apps/pinnacle-notify ~amd64
gui-apps/pinnacle-lock ~amd64
gui-apps/pinnacle-screenshot ~amd64
gui-apps/pinnacle-fm ~amd64
gui-apps/pinnacle-translator ~amd64
gui-apps/crucian ~amd64
###For those who decide to use pkgrs:
dev-rust/* ~amd64
dev-rust/*-* ~amd64

```
2. Overlay Setup
Preparation of the overlay is included in the ebuild, according to the logic: The binary file has been launched 🦀-->a configuration has been created in the repository.

```
eix-update
sudo emerge --ask app-portage/tupoll-overlay
sudo emerge -av pinnacle-terminal pinnacle-wallpaper pinnacle-notify pinnacle-lock pinnacle-screenshot pinnacle-fm pinnacle-translator

```
3. File System Preparation
The following command adds an entry to /etc/fstab (if missing), allocates 10MB from tmpfs, and sets user permissions:
```
sudo fstab-tmpfs-config 

```
4. Install Pinnacle
```
emerge gui-wm/pinnacle gui-wm/pinnacle-gentoo

```
5. WM & Ironbar Configuration
Run this command as a regular user:
```
pinnacle-gentoo

```

6. Keyboard Layout Widget
```
sudo helper_kbd 

```
7. Application Configuration
Configure other programs using these commands:
```
thunar-conf
fish-conf
mako-conf

```
To set Fish as your default shell and add useful aliases:
```
echo "exec fish" > $HOME/.bashrc
alias reboot "sudo reboot"
funcsave reboot
alias poweroff "sudo /sbin/poweroff"
funcsave poweroff

```
Note: If you use the Fish rule, update your sudoers file:
```
Cmnd_Alias REBOOT = /sbin/halt, /sbin/reboot, /sbin/poweroff
root ALL=(ALL) ALL
<your_username> ALL=(ALL) ALL, NOPASSWD: REBOOT 

```
System Binaries (/usr/bin)

| Component | Description |
| :--- | :--- |
| **cpu_oval** | CPU usage widget |
| **bottom_bar** | Bottom panel |
| **cpu_temp_oval** | CPU temperature widget |
| **fstab-tmpfs-config** | File system configuration tool |
| **ironbar-config** | Ironbar configuration generator |
| **kbd-rs** | Keyboard layout widget |
| **left_bar** | Left panel |
| **memory_oval** | RAM usage widget |
| **netgraph / rxgraph / txgraph** | Network traffic widgets |
| **vol_widget** | ALSA volume control |
| **helper_kbd** | Keyboard Widget Helper |
| **pinnacle-terminal** | Terminal |
| **pinnacle-wallpaper** | Desktop wallpaper  |
| **pinnacle-translator** | Translator |
| **pinnacle-lock** | Screen lock |
| **pinnacle-notify** | Service notifications |
| **pinnacle-screenshot** | Screenshot program |
| **pinnacle-fm** | File Manager |
| **crucian** | Editor and AI assistant |
| **servo** | Rust web-brawser Servo |
| **servomenu** | Servoshell menu GTk4 |
To install a package from the rust repository:
```
pkgrs -sc wayshot
dev-rust/wayshot-1.4.6
      Latest version available: [from cache]
      Description: Screenshot crate for wlroots based compositors implementing the zwlr_screencopy…
pkgrs -wb wayshot 1.4.6
pkgrs -i --ask dev-rust/wayshot

```
For different cargo packages there are different ways to create an ebuild. Read pkgrs-man.

```
cargo install wayshot
git clone https://github.com/jtroo/kanata.git
cd kanata
cargo build --release --no-default-features --features "cmd"
```
Wallpapers
```
Default wallpapers are located in /usr/share/pinnacle-gentoo/pictures:
tag1.jpg through tag6.jpg.

```

WARNING
🔥Be careful! Some files like libzfs or efistab might appear in the overlay profiles. Check:
/var/db/repos/tupoll-overlay/profiles/targets/amd64/wayland/make.defaults
Usage Notes
First Launch: When running pinnacle --session for the first time, the screen may remain white while the config is compiled from GitHub.
Auto-login Tuning: If you want to bypass the display manager, refer to the Gentoo Wiki and remove the display manager init:
```
emerge -aC gui-libs/display-manager-init

```
Session Lock: To lock the session on startup, edit $HOME/.config/pinnacle/src/main.rs. After the pinnacle-notify spawn line, add:

```
Command::with_shell(["fish", "-c"], "pinnacle-lock").once().spawn();

```
To start the window manager you will now have to create the file /usr/local/bin/start_pinnacle
change the bit number of the file, or write it for sudo:
```
#!/bin/fish
mkdir -p /run/user
chmod 1777 /run/user
mkdir -p /run/user/1000
# 2. Даем права вашему пользователю
chown 1000:1000 /run/user/1000
# 3. Устанавливаем права доступа (важно для безопасности XDG)
chmod 700 /run/user/1000
###Make sure your user is 1000
pinnacle --session

```
PKGRS - PACKAGE MANAGEMENT SYSTEM (RUST + GENTOO)

DESCRIPTION:
  pkgrs is a wrapper around Portage that seamlessly integrates
  system repositories with the Cargo ecosystem. When searching
  data is displayed from the local SQLite database (emerge-cargo-base).
  
## 💖 Credits

A huge shout-out to the developers of these awesome projects:

* 🏔️ **[Pinnacle WM](https://pinnacle-comp.github.io/pinnacle/getting-started/introduction.html)** — for the solid foundation and window management magic.
* ⚡ **[Ironbar](https://github.com/JakeStanger/ironbar.git)** — for the slickest bar in the West.

This project wouldn't be the same without your hard work! 🙌

