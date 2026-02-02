# tupoll-overlay
The overlay is added with a simple Rust installation script and is ready to use.
Add your ebuids to it; it's local.

To install tupoll-overlay for gentoo, use the github:
```
git clone https://github.com/tupoll/tupoll-overlay.git
```
Install using the setup program (don't forget to enter the root password ):
```
sudo mkdir -p /var/db/repos/tupoll-overlay/app-portage/
sudo mv -f $HOME/tupoll-overlay /var/db/repos/tupoll-overlay/app-portage/
sudo chown -R portage:portage /var/db/repos/tupoll-overlay
sudo ebuild /var/db/repos/tupoll-overlay/app-portage/tupoll-overlay/tupoll-overlay-9999.ebuild manifest
sudo eix-update
sudo emerge -av app-portage/tupoll-overlay
sudo /usr/bin/tupoll-overlay
```

Clean and delete the directory $HOME 🗑:
```
rm -rf $HOME/tupoll-overlay

```


For example, to install the Pinnacle window manager:
```
pinnacle-conifg
sudo eix-update
emerge --ask gui-wm/pinnacle
```
🌝
