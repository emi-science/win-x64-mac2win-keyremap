use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};

use log::{error, info};
use once_cell::sync::Lazy;
use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        UI::{
            Input::KeyboardAndMouse::{
                SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
                VK_LCONTROL, VK_LWIN, VK_RCONTROL, VK_RWIN,
            },
            WindowsAndMessaging::{
                CallNextHookEx, GetMessageW, HC_ACTION, HHOOK, KBDLLHOOKSTRUCT, MSG,
                SetWindowsHookExW, UnhookWindowsHookEx, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
            },
        },
    },
};

// Global variable to keep track of the hook
static HOOK_HANDLE: Lazy<Mutex<Option<HHOOK>>> = Lazy::new(|| Mutex::new(None));
static RUNNING: AtomicBool = AtomicBool::new(true);
// Track if Command/Win keys are being held down
static LEFT_WIN_PRESSED: AtomicBool = AtomicBool::new(false);
static RIGHT_WIN_PRESSED: AtomicBool = AtomicBool::new(false);

// The keyboard hook callback function
extern "system" fn keyboard_hook_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code < 0 {
        // Pass to the next hook if code is negative
        return unsafe { CallNextHookEx(None, code, wparam, lparam) };
    } else if code == HC_ACTION as i32 {
        let kb_struct = unsafe { *(lparam.0 as *const KBDLLHOOKSTRUCT) };
        let key_code = kb_struct.vkCode;

        // Check if the key is Win key (Command key on Mac keyboards)
        let is_cmd_key = key_code == VK_LWIN.0 as u32 || key_code == VK_RWIN.0 as u32;

        // Determine if it's a key press or release
        let is_key_down = wparam.0 == WM_KEYDOWN as usize;
        let is_key_up = wparam.0 == WM_KEYUP as usize;

        // Update Command/Win key state
        if is_cmd_key {
            if key_code == VK_LWIN.0 as u32 {
                LEFT_WIN_PRESSED.store(is_key_down, Ordering::SeqCst);
            } else {
                RIGHT_WIN_PRESSED.store(is_key_down, Ordering::SeqCst);
            }

            if is_key_down || is_key_up {
                // Map Left Win to Left Ctrl, Right Win to Right Ctrl
                let ctrl_key = if key_code == VK_LWIN.0 as u32 {
                    VK_LCONTROL
                } else {
                    VK_RCONTROL
                };

                // Simulate Ctrl key press or release
                send_keyboard_input(ctrl_key, is_key_up);

                // Return 1 to prevent the system from passing the command key event to applications
                return LRESULT(1);
            }
        } 
        // Check if any other key is pressed while Command/Win is being held
        else if LEFT_WIN_PRESSED.load(Ordering::SeqCst) || RIGHT_WIN_PRESSED.load(Ordering::SeqCst) {
            // For Command+Key combinations, we still want the Ctrl+Key functionality to work
            // but prevent the original Command+Key default behavior
            
            // Allow the Ctrl+Key combination to go through naturally
            // (we don't need to manually send it because the Win→Ctrl remapping is already active)
            
            // Return 1 to prevent the system from processing the Command+Key combination with default behavior
            return LRESULT(1);
        }
    }

    // Pass to the next hook
    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

// Helper function to send keyboard input events
fn send_keyboard_input(key_code: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY, is_key_up: bool) {
    let flags = if is_key_up {
        KEYEVENTF_KEYUP
    } else {
        KEYBD_EVENT_FLAGS(0)
    };

    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: windows::Win32::UI::Input::KeyboardAndMouse::INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key_code,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    let mut inputs = [input];

    unsafe {
        SendInput(&mut inputs, mem::size_of::<INPUT>() as i32);
    }
}

// Install the keyboard hook
fn install_keyboard_hook() -> Result<(), String> {
    let hook = unsafe {
        SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(keyboard_hook_proc),
            None,
            0,
        )
    };

    if hook.is_err() {
        return Err("Failed to install keyboard hook".to_string());
    }

    let mut hook_handle = HOOK_HANDLE.lock().unwrap();
    *hook_handle = Some(hook.unwrap());
    Ok(())
}

// Remove the keyboard hook
fn uninstall_keyboard_hook() {
    let mut hook_handle = HOOK_HANDLE.lock().unwrap();
    if let Some(hook) = *hook_handle {
        unsafe {
            UnhookWindowsHookEx(hook);
        }
        *hook_handle = None;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    // Initialize logging
    env_logger::init();
    info!("Mac Command to Windows CTRL Key Remapper starting...");

    // Set up Ctrl+C handler for graceful shutdown
    ctrlc::set_handler(|| {
        info!("Received termination signal. Shutting down...");
        RUNNING.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    // Install keyboard hook
    match install_keyboard_hook() {
        Ok(_) => info!("Keyboard hook installed successfully"),
        Err(e) => {
            error!("Failed to install keyboard hook: {}", e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                e,
            )));
        }
    }

    info!("Mac Command → Windows CTRL remapper is now running...");
    info!("Press Ctrl+C to exit");

    // Message loop - keep the application running and processing Windows messages
    let mut msg = MSG::default();
    while RUNNING.load(Ordering::SeqCst) {
        // GetMessageW will block until there's a message
        let result = unsafe { GetMessageW(&mut msg, HWND::default(), 0, 0) };
        
        if result.0 == 0 {
            // WM_QUIT message received
            break;
        }
    }

    // Clean up
    info!("Removing keyboard hook...");
    uninstall_keyboard_hook();
    info!("Mac Command to Windows CTRL Key Remapper shut down successfully");

    Ok(())
}
