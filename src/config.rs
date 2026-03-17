pub const DB_PATH: &str = ".passwords.sqlite3";

pub fn db_exists() -> bool {
    std::path::Path::new(DB_PATH).exists()
}