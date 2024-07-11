use std::sync::{LazyLock, Mutex};

use windows_sys::Win32::{
    Foundation::{HWND, LPARAM, RECT},
    UI::WindowsAndMessaging::{EnumWindows, GetWindowRect, GetWindowThreadProcessId},
};

pub fn get_resolution(process_id: *mut u32) -> Option<[i32; 2]> {
    let lparam = unsafe { *process_id as LPARAM };

    unsafe {
        EnumWindows(Some(window_callback), lparam);
    }

    let hwnd = *HWND.lock().unwrap();

    #[cfg(test)]
    dbg!(hwnd);

    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    match hwnd != 0 && unsafe { GetWindowRect(hwnd, &mut rect) } != 0 {
        true => {
            #[cfg(test)]
            dbg!(hwnd, rect.right, rect.left, rect.bottom, rect.top);

            Some([rect.right - rect.left, rect.bottom - rect.top])
        }
        false => None,
    }
}

pub static HWND: LazyLock<Mutex<HWND>> = LazyLock::new(|| Mutex::new(0 as HWND));

unsafe extern "system" fn window_callback(hwnd: HWND, l_param: LPARAM) -> i32 {
    let mut lpdw_process_id = 0;
    GetWindowThreadProcessId(hwnd, &mut lpdw_process_id);

    if lpdw_process_id == l_param as u32 {
        *HWND.lock().unwrap() = hwnd;
        return 0;
    }

    1
}

#[cfg(test)]
mod tests {
    use crate::statics::PID;

    use super::*;

    #[test]
    fn test_get_resolution() {
        let resolution = get_resolution(&mut PID.to_owned());

        dbg!(resolution);

        assert!(resolution.is_some());
    }
}
