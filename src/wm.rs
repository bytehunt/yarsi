use std::env;
/// Get current window manager(or DE) using envvars
pub fn get_wm() -> Option<String> {
    let mut wm = env::var("DESKTOP_SESSION")
        .or_else(|_| env::var("XDG_CURRENT_DESKTOP"))
        .or_else(|_| env::var("WINDOWMANAGER"))
        .ok()?;
    if wm.starts_with('/') {
        wm = extract_file_from_path(&wm)?;
    }
    Some(wm)
}

fn extract_file_from_path(path: impl ToString) -> Option<String> {
    let path = path.to_string();
    let split: Vec<_> = path.split('/').collect();
    split.last().map(|p| p.to_string())
}
