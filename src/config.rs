use std::path::PathBuf;

pub const DB_NAME: &str = ".passwords.sqlite3";

pub fn get_db_path() -> PathBuf {
    if cfg!(debug_assertions) {
        std::env::current_dir().expect("Failed to get current dir").join(DB_NAME)
    } else {
        let exe_path = std::env::current_exe().expect("Failed to get current exe path");
        let exe_dir = exe_path.parent().expect("Failed to get exe directory");
        exe_dir.join(DB_NAME)
    }
}
