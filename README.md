# tupoll-overlay
The overlay is added with a simple Rust installation script and is ready to use.
Add your ebuids to it; it's local.

To install tupoll-overlay for gentoo, use the github:
```
git clone 
```
Install using the setup program (don't forget to enter the root password when asked):
```
cd tupoll-overlay
sudo -s
cargo build --release
cd target/release
./tupoll-overlay
```

Clean and delete the directory 🗑:
```
cd $HOME/tupoll-overlay
sudo cargo clean
rm -rf tupoll-overlay

```
For example, to install the Pinnacle window manager:
```
sudo eix-update
emerge --ask gui-wm/pinnacle
```
🌝
