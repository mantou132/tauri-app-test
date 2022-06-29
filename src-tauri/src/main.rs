#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .on_window_event(|event| match event.event() {
        tauri::WindowEvent::Resized(_) => {
            println!("is_fullscreen {:?}", event.window().is_fullscreen());
        }
        _ => {}
    })
    .run(context)
    .expect("error while running tauri application");
}
