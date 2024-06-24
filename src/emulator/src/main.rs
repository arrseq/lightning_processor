use std::ffi::c_void;

use windows::{
    core::*, Win32::{Foundation::*, Graphics::{Dwm::{DwmSetWindowAttribute, DWMWA_TEXT_COLOR, DWMWINDOWATTRIBUTE}, Gdi::ValidateRect}, System::LibraryLoader::GetModuleHandleA, UI::WindowsAndMessaging::*}
};

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(None)?;
        let window_class = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance.into(),
            lpszClassName: window_class,

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("This is a sample window"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        );

        let mut message = MSG::default();

        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                _ = ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            },
            WM_MOUSEWHEEL => {
                println!("Mouse Wheel Moved {:?}, {:?}", wparam, lparam);

                SetWindowTextW(window, w!("Mouse Scrolled")).unwrap();
                let val = COLORREF(0x00FFFF00);
                DwmSetWindowAttribute(window, DWMWA_TEXT_COLOR, &val as *const COLORREF as *const _, std::mem::size_of::<COLORREF>() as _).unwrap();

                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}