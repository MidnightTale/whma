#![windows_subsystem = "windows"]
mod config;

use windows::{
    Win32::Foundation::*, 
    Win32::UI::WindowsAndMessaging::*,
    Win32::Graphics::Gdi::*,
    Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId},
};
use std::time::{Duration, Instant};
use config::Config;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

fn is_blacklisted(window: HWND, config: &Config) -> bool {
    unsafe {
        let mut title_buffer = [0u16; 512];
        let mut class_buffer = [0u16; 512];
        
        // Check window title
        let title_len = GetWindowTextW(window, &mut title_buffer);
        if title_len > 0 {
            let window_title = OsString::from_wide(&title_buffer[..title_len as usize])
                .to_string_lossy()
                .to_string();
            
            if config.blacklist.iter().any(|entry| !entry.window_title.is_empty() 
                && window_title.contains(&entry.window_title)) {
                return true;
            }
        }

        // Check class name
        let class_len = GetClassNameW(window, &mut class_buffer);
        if class_len > 0 {
            let class_str = OsString::from_wide(&class_buffer[..class_len as usize])
                .to_string_lossy()
                .to_string();
            
            if config.blacklist.iter().any(|entry| !entry.class_name.is_empty() 
                && class_str == entry.class_name) {
                return true;
            }
        }

        false
    }
}

fn main() {
    let config = Config::load();
    
    unsafe {
        let mut point = POINT::default();
        let mut last_window = HWND(0);
        let mut last_switch = Instant::now();
        let mut last_monitor = HMONITOR(0);
        let window_flags = SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW;

        loop {
            GetCursorPos(&mut point);
            let window = WindowFromPoint(point);
            
            // Dynamic sleep duration based on system activity and window state
            let sleep_duration = if Instant::now().elapsed() < Duration::from_millis(1000) {
                if window == last_window {
                    config.delay_ms * 2
                } else {
                    config.delay_ms
                }
            } else {
                config.delay_ms * 3
            };

            if !config.enabled {
                std::thread::sleep(Duration::from_millis(500));
                continue;
            }
            
            let current_monitor = MonitorFromPoint(point, MONITOR_DEFAULTTONEAREST);
            
            // Early exit conditions
            if window.0 == 0 || (window == last_window && current_monitor == last_monitor) {
                std::thread::sleep(Duration::from_millis(sleep_duration));
                continue;
            }

            if !is_blacklisted(window, &config) {
                let mut placement = WINDOWPLACEMENT::default();
                placement.length = std::mem::size_of::<WINDOWPLACEMENT>() as u32;
                
                if GetWindowPlacement(window, &mut placement).as_bool() 
                    && placement.showCmd != SHOW_WINDOW_CMD(SW_SHOWMINIMIZED.0)
                    && last_switch.elapsed() > Duration::from_millis(config.cooldown_ms)
                {
                    let foreground = GetForegroundWindow();
                    let mut process_id = 0u32;
                    let thread_id = GetWindowThreadProcessId(foreground, Some(&mut process_id));
                    let current_thread = GetCurrentThreadId();

                    AllowSetForegroundWindow(ASFW_ANY);
                    AttachThreadInput(current_thread, thread_id, true);
                    
                    SetWindowPos(
                        window,
                        HWND_TOPMOST,
                        0, 0, 0, 0,
                        window_flags | SWP_NOACTIVATE
                    );
                    
                    SetForegroundWindow(window);
                    
                    SetWindowPos(
                        window,
                        HWND_NOTOPMOST,
                        0, 0, 0, 0,
                        window_flags
                    );
                    
                    AttachThreadInput(current_thread, thread_id, false);
                    
                    last_window = window;
                    last_monitor = current_monitor;
                    last_switch = Instant::now();
                }
            }

            std::thread::sleep(Duration::from_millis(sleep_duration));
        }
    }
}