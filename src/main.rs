use directories::{ProjectDirs};

fn main() {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Techulus",  "DropZone") {
        let dir = proj_dirs.config_dir();
        println!("Config path -> {:?}", dir);
    }
}
