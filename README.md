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
sudo mkdir -p /var/db/repos/tupoll-overlay/app-portage/
sudo mv -f $HOME/tupoll-overlay /var/db/repos/tupoll-overlay/app-portage/
sudo chown -R portage:portage /var/db/repos/tupoll-overlay
sudo ebuild /var/db/repos/tupoll-overlay/app-portage/tupoll-overlay/tupoll-overlay-9999.ebuild manifest
sudo eix-update
sudo emerge -av app-portage/tupoll-overlay
sudo /usr/bin/tupoll-overlay

```

Clean up the installation directory $HOME 🗑:
```
rm -rf $HOME/tupoll-overlay

```

Configuration & Setup
Before launching the WM, follow these steps:
1. Keywords Configuration
Add the following entries to /etc/portage/package.accept_keywords/:
```
media-video/soxbar **
app-portage/tupoll-overlay ** 
gui-wm/pinnacle **
gui-wm/pinnacle-gentoo ** 

```
2. Overlay Setup
Add the ebuild to the overlay:
```
sudo pinnacle-config

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
| **wp-updater** | Wallpaper updater utility |


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
Session Lock: To lock the session on startup, edit $HOME/.config/pinnacle/src/main.rs. After the mako spawn line, add:

```
Command::with_shell(["fish", "-c"], "swaylock -f -C ~/.config/swaylock/config").once().spawn();

```
##Credits
Special thanks to the developers of these projects:
Pinnacle WM
Ironbar
