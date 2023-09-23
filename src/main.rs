use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::{Input::KeyboardAndMouse::VK_ESCAPE, WindowsAndMessaging::*},
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

        RegisterClassA(&wc);

        let hwnd = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("Git User Switch"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            280,
            400,
            None,
            None,
            instance,
            None,
        );

        ShowWindow(hwnd, SW_SHOWDEFAULT);

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
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_KEYDOWN => {
                // if wparam.0 == VK_ESCAPE.into() as usize {
                //     ShowWindow(window, SW_HIDE);
                // }
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
