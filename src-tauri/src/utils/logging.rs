pub fn log_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_default()
}