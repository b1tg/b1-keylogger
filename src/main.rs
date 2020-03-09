extern crate winapi;
extern crate user32;
//use std::cast;
use std::io::Error;
use winapi::um::winuser::{KBDLLHOOKSTRUCT,  CallNextHookEx};
//use winapi::shared::minwindef::LPARAM;
use winapi::shared::windef::HHOOK__;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winuser::GetMessageW;
use winapi::um::winuser::{LPMSG, MSG};
use winapi::shared::minwindef::{BOOL, TRUE, FALSE};
const WH_KEYBOARD_LL: i32 = 13;
const WM_KEYDOWN: u64 = 0x0100;
type LPARAM = isize;
//struct Foo;
//static hook_id:HHOOK__  = unsafe { cast::transmute(Foo) };
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}

fn main() {
    //print_message("Hello, wwww!").unwrap();
    unsafe {
        println!("before hook");
        let hook_id =
            user32::SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_callback), std::ptr::null_mut(), 0);

        println!("hook_id{:?}", hook_id);
        // Don't forget to release the hook eventually
        //  user32::UnhookWindowsHookEx(hook_id);
        // let mut msg:MSG = MSG::new() ;
        use winapi::shared::windef::{HWND, POINT};
        use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, DWORD};
         let mut msg = MSG {
            hwnd : 0 as HWND,
            message : 0 as UINT,
            wParam : 0 as WPARAM,
            lParam : 0 as LPARAM,
            time : 0 as DWORD,
            pt : POINT { x: 0, y: 0, },
        };
        while GetMessageW( &mut msg as LPMSG ,  std::ptr::null_mut(), 0, 0)!=FALSE
        {
        }
    }
}


extern "system" fn hook_callback(code: i32, wParam: u64, lParam: i64) -> i64 {

    //println!("hhhh11111\n");
    // print_message("ook");
    if code >=0 {
        if wParam == WM_KEYDOWN {
            let kbdStruct = unsafe {(*(lParam as *mut KBDLLHOOKSTRUCT)) as KBDLLHOOKSTRUCT};
            println!("{:?}", kbdStruct.vkCode);
        }
    }
    //let res = unsafe{ CallNextHookEx(hook_id, code, wParam as usize, lParam as isize)};
    return 0;
}
