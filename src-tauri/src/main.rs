#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ron::Value as RONValue;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSONValue;
use serde_yaml::Value as YAMLValue;
use toml::Value as TOMLValue;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowButton, NSWindowStyleMask, NSWindowTitleVisibility};

#[cfg(target_os = "macos")]
use objc::runtime::YES;
use tauri::{Manager, RunEvent, Runtime, Window, WindowEvent};

pub trait WindowExt {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, title_transparent: bool, remove_toolbar: bool);
}

impl<R: Runtime> WindowExt for Window<R> {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, title_transparent: bool, remove_tool_bar: bool) {
        unsafe {
            let id = self.ns_window().unwrap() as cocoa::base::id;
            NSWindow::setTitlebarAppearsTransparent_(id, cocoa::base::YES);
            let mut style_mask = id.styleMask();
            style_mask.set(
                NSWindowStyleMask::NSFullSizeContentViewWindowMask,
                title_transparent,
            );
            id.setStyleMask_(style_mask);
            if remove_tool_bar {
                let close_button = id.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
                let _: () = msg_send![close_button, setHidden: YES];
                let min_button =
                    id.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
                let _: () = msg_send![min_button, setHidden: YES];
                let zoom_button = id.standardWindowButton_(NSWindowButton::NSWindowZoomButton);
                let _: () = msg_send![zoom_button, setHidden: YES];
            }
            id.setTitleVisibility_(if title_transparent {
                NSWindowTitleVisibility::NSWindowTitleHidden
            } else {
                NSWindowTitleVisibility::NSWindowTitleVisible
            });
            id.setTitlebarAppearsTransparent_(if title_transparent {
                cocoa::base::YES
            } else {
                cocoa::base::NO
            });
        }
    }
}

#[derive(Deserialize)]
enum Format {
    Json,
    Toml,
    Yaml,
    Ron,
    Xml,
    Url,
}

fn convert_t<T: Serialize>(input: T, format: Format) -> Option<String> {
    let output = match format {
        Format::Json => serde_json::to_string_pretty(&input).ok(),
        Format::Toml => toml::to_string_pretty(&input).ok(),
        Format::Yaml => serde_yaml::to_string(&input).ok(),
        Format::Ron => ron::to_string(&input).ok(),
        Format::Xml => serde_xml_rs::to_string(&input).ok(),
        Format::Url => serde_urlencoded::to_string(&input).ok(),
    };

    output
}

fn convert_raw(
    input: &str,
    input_type: Format,
    output_type: Format,
) -> Result<Option<String>, anyhow::Error> {
    Ok(match input_type {
        Format::Json => convert_t(serde_json::from_str::<JSONValue>(input)?, output_type),
        Format::Toml => convert_t(toml::from_str::<TOMLValue>(&input)?, output_type),
        Format::Yaml => convert_t(serde_yaml::from_str::<YAMLValue>(&input)?, output_type),
        Format::Ron => convert_t(ron::from_str::<RONValue>(&input)?, output_type),
        Format::Xml => convert_t(serde_xml_rs::from_str(&input)?, output_type),
        Format::Url => convert_t(serde_urlencoded::from_str(&input)?, output_type),
    })
}

#[tauri::command]
fn convert(input: &str, int: Format, out: Format) -> Result<String, bool> {
    let converted = convert_raw(input, int, out);
    let Ok(converted) = converted else {
        return Err(true)
    };
    let Some(converted) = converted else {
        return Err(false)
    };
    #[cfg(debug_assertions)]
    println!("{converted}");
    Ok(converted)
}

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(target_os = "macos")]
            window.set_transparent_titlebar(true, false);

            #[cfg(debug_assertions)]
            window.open_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![convert])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app_handle, e| match e {
        RunEvent::WindowEvent {
            label,
            event: WindowEvent::CloseRequested { api, .. },
            ..
        } => {
            if label == "main" {
                let app_handle = app_handle.clone();
                let window = app_handle.get_window(&label).unwrap();
                api.prevent_close();
                window
                    .emit_all("close-requested", "Requesting close.")
                    .unwrap();
                window.once("close-safe", |_| {
                    #[cfg(debug_assertions)]
                    println!("Saved");
                    std::thread::spawn(move || {
                        app_handle.get_window(&label).unwrap().close().unwrap();
                    });
                });
            }
        }
        _ => {}
    })
}
