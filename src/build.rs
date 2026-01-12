const COMMANDS: &[&str] = &["connect", "create", "get_by_id"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
