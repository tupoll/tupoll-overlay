use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = "/var/db/repos/tupoll-overlay/profiles/targets/amd64/wayland/use.mask";
    
    let content = r#"systemd
pulseaudio
-x264
-gnome-keyring
-fuse
 mmal
-python_single_target_python3_12
-python_targets_python3_13
-efistub

"#;

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    println!("Успешно записан.");
    Ok(())
}
