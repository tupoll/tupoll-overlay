use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = "/var/db/repos/tupoll-overlay/profiles/profiles.desc";
    
    let content = r#"#############################################
# This is a list of valid profiles for each architecture.  This file is used by
# repoman when doing a repoman scan or repoman full.
# DO NOT ADD PROFILES WITH A "die" or "exit" IN THEM OR IT KILLS REPOMAN
#
#layout:
#arch		profile_directory				status

# These profiles will augment those already present in the
# core gentoo repo

# ARM-64 Profiles (retain rpi3 for now, but shift to equivalent genpi64)
#arm             default/linux/arm/23.0/desktop/rpi3             dev
#arm64           default/linux/arm64/17.0/desktop/rpi3           dev
amd64           default/linux/amd64/23.0/desktop/wayland           dev

"#;

    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, content)?;

    println!("Успешно записан.");
    Ok(())
}
