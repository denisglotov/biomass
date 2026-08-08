#[cfg(not(target_arch = "wasm32"))]
fn get_save_file_paths() -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();

    #[cfg(target_os = "android")]
    {
        let android_files = std::path::PathBuf::from("/data/data/org.dymka.biomass/files");
        if std::fs::create_dir_all(&android_files).is_ok() {
            paths.push(android_files.join("biomass_save.txt"));
        }
        paths.push(std::path::PathBuf::from(
            "/data/data/org.dymka.biomass/biomass_save.txt",
        ));
    }

    if let Ok(home) = std::env::var("HOME") {
        let home_path = std::path::PathBuf::from(home);
        paths.push(home_path.join(".biomass_save.txt"));
    }

    paths.push(std::env::temp_dir().join("biomass_save.txt"));
    paths.push(std::path::PathBuf::from("biomass_save.txt"));

    paths
}

pub fn save_last_level_reached(level_idx: usize) {
    let current_highest = load_last_level_reached();
    let highest = level_idx.max(current_highest);

    #[cfg(not(target_arch = "wasm32"))]
    {
        for path in get_save_file_paths() {
            if std::fs::write(&path, highest.to_string()).is_ok() {
                break;
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    let _ = highest;
}

pub fn load_last_level_reached() -> usize {
    #[cfg(not(target_arch = "wasm32"))]
    {
        for path in get_save_file_paths() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(idx) = content.trim().parse::<usize>() {
                    return idx;
                }
            }
        }
        0
    }

    #[cfg(target_arch = "wasm32")]
    0
}
