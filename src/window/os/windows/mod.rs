use crate::{
    win32,
    window::{
        check_buffer_size, error::Error, icon::Icon, key_handler::KeyHandler, rate::UpdateRate,
        CursorStyle, InputCallback, Key, KeyRepeat, MouseButton, MouseMode, Result, Scale,
        ScaleMode, WindowOptions,
    },
};
use std::{
    ffi::{c_void, OsStr},
    os::windows::ffi::OsStrExt,
    time::Duration,
};

fn update_key_state(window: &mut Window, wparam: u32, state: bool) {
    match wparam & 0x1ff {
        0x00B => window.key_handler.set_key_state(Key::Key0, state),
        0x002 => window.key_handler.set_key_state(Key::Key1, state),
        0x003 => window.key_handler.set_key_state(Key::Key2, state),
        0x004 => window.key_handler.set_key_state(Key::Key3, state),
        0x005 => window.key_handler.set_key_state(Key::Key4, state),
        0x006 => window.key_handler.set_key_state(Key::Key5, state),
        0x007 => window.key_handler.set_key_state(Key::Key6, state),
        0x008 => window.key_handler.set_key_state(Key::Key7, state),
        0x009 => window.key_handler.set_key_state(Key::Key8, state),
        0x00A => window.key_handler.set_key_state(Key::Key9, state),
        0x01E => window.key_handler.set_key_state(Key::A, state),
        0x030 => window.key_handler.set_key_state(Key::B, state),
        0x02E => window.key_handler.set_key_state(Key::C, state),
        0x020 => window.key_handler.set_key_state(Key::D, state),
        0x012 => window.key_handler.set_key_state(Key::E, state),
        0x021 => window.key_handler.set_key_state(Key::F, state),
        0x022 => window.key_handler.set_key_state(Key::G, state),
        0x023 => window.key_handler.set_key_state(Key::H, state),
        0x017 => window.key_handler.set_key_state(Key::I, state),
        0x024 => window.key_handler.set_key_state(Key::J, state),
        0x025 => window.key_handler.set_key_state(Key::K, state),
        0x026 => window.key_handler.set_key_state(Key::L, state),
        0x032 => window.key_handler.set_key_state(Key::M, state),
        0x031 => window.key_handler.set_key_state(Key::N, state),
        0x018 => window.key_handler.set_key_state(Key::O, state),
        0x019 => window.key_handler.set_key_state(Key::P, state),
        0x010 => window.key_handler.set_key_state(Key::Q, state),
        0x013 => window.key_handler.set_key_state(Key::R, state),
        0x01F => window.key_handler.set_key_state(Key::S, state),
        0x014 => window.key_handler.set_key_state(Key::T, state),
        0x016 => window.key_handler.set_key_state(Key::U, state),
        0x02F => window.key_handler.set_key_state(Key::V, state),
        0x011 => window.key_handler.set_key_state(Key::W, state),
        0x02D => window.key_handler.set_key_state(Key::X, state),
        0x015 => window.key_handler.set_key_state(Key::Y, state),
        0x02C => window.key_handler.set_key_state(Key::Z, state),
        0x03B => window.key_handler.set_key_state(Key::F1, state),
        0x03C => window.key_handler.set_key_state(Key::F2, state),
        0x03D => window.key_handler.set_key_state(Key::F3, state),
        0x03E => window.key_handler.set_key_state(Key::F4, state),
        0x03F => window.key_handler.set_key_state(Key::F5, state),
        0x040 => window.key_handler.set_key_state(Key::F6, state),
        0x041 => window.key_handler.set_key_state(Key::F7, state),
        0x042 => window.key_handler.set_key_state(Key::F8, state),
        0x043 => window.key_handler.set_key_state(Key::F9, state),
        0x044 => window.key_handler.set_key_state(Key::F10, state),
        0x057 => window.key_handler.set_key_state(Key::F11, state),
        0x058 => window.key_handler.set_key_state(Key::F12, state),
        0x150 => window.key_handler.set_key_state(Key::Down, state),
        0x14B => window.key_handler.set_key_state(Key::Left, state),
        0x14D => window.key_handler.set_key_state(Key::Right, state),
        0x148 => window.key_handler.set_key_state(Key::Up, state),
        0x028 => window.key_handler.set_key_state(Key::Apostrophe, state),
        0x029 => window.key_handler.set_key_state(Key::Backquote, state),
        0x02B => window.key_handler.set_key_state(Key::Backslash, state),
        0x033 => window.key_handler.set_key_state(Key::Comma, state),
        0x00D => window.key_handler.set_key_state(Key::Equal, state),
        0x01A => window.key_handler.set_key_state(Key::LeftBracket, state),
        0x00C => window.key_handler.set_key_state(Key::Minus, state),
        0x034 => window.key_handler.set_key_state(Key::Period, state),
        0x01B => window.key_handler.set_key_state(Key::RightBracket, state),
        0x027 => window.key_handler.set_key_state(Key::Semicolon, state),
        0x035 => window.key_handler.set_key_state(Key::Slash, state),
        0x00E => window.key_handler.set_key_state(Key::Backspace, state),
        0x153 => window.key_handler.set_key_state(Key::Delete, state),
        0x14F => window.key_handler.set_key_state(Key::End, state),
        0x01C => window.key_handler.set_key_state(Key::Enter, state),
        0x001 => window.key_handler.set_key_state(Key::Escape, state),
        0x147 => window.key_handler.set_key_state(Key::Home, state),
        0x152 => window.key_handler.set_key_state(Key::Insert, state),
        0x15D => window.key_handler.set_key_state(Key::Menu, state),
        0x151 => window.key_handler.set_key_state(Key::PageDown, state),
        0x149 => window.key_handler.set_key_state(Key::PageUp, state),
        0x045 => window.key_handler.set_key_state(Key::Pause, state),
        0x039 => window.key_handler.set_key_state(Key::Space, state),
        0x00F => window.key_handler.set_key_state(Key::Tab, state),
        0x145 => window.key_handler.set_key_state(Key::NumLock, state),
        0x03A => window.key_handler.set_key_state(Key::CapsLock, state),
        0x046 => window.key_handler.set_key_state(Key::ScrollLock, state),
        0x02A => window.key_handler.set_key_state(Key::LeftShift, state),
        0x036 => window.key_handler.set_key_state(Key::RightShift, state),
        0x01D => window.key_handler.set_key_state(Key::LeftCtrl, state),
        0x11D => window.key_handler.set_key_state(Key::RightCtrl, state),
        0x052 => window.key_handler.set_key_state(Key::NumPad0, state),
        0x04F => window.key_handler.set_key_state(Key::NumPad1, state),
        0x050 => window.key_handler.set_key_state(Key::NumPad2, state),
        0x051 => window.key_handler.set_key_state(Key::NumPad3, state),
        0x04B => window.key_handler.set_key_state(Key::NumPad4, state),
        0x04C => window.key_handler.set_key_state(Key::NumPad5, state),
        0x04D => window.key_handler.set_key_state(Key::NumPad6, state),
        0x047 => window.key_handler.set_key_state(Key::NumPad7, state),
        0x048 => window.key_handler.set_key_state(Key::NumPad8, state),
        0x049 => window.key_handler.set_key_state(Key::NumPad9, state),
        0x053 => window.key_handler.set_key_state(Key::NumPadDot, state),
        0x135 => window.key_handler.set_key_state(Key::NumPadSlash, state),
        0x037 => window.key_handler.set_key_state(Key::NumPadAsterisk, state),
        0x04A => window.key_handler.set_key_state(Key::NumPadMinus, state),
        0x04E => window.key_handler.set_key_state(Key::NumPadPlus, state),
        0x11C => window.key_handler.set_key_state(Key::NumPadEnter, state),
        _ => (),
    }
}

#[inline]
fn char_down(window: &mut Window, code_point: u32) {
    if let Some(ref mut callback) = window.key_handler.key_callback {
        callback.add_char(code_point);
    }
}

#[cfg(target_arch = "x86_64")]
#[inline]
unsafe fn set_window_long(window: win32::HWND, data: win32::LONG_PTR) -> win32::LONG_PTR {
    win32::SetWindowLongPtrW(window, win32::GWLP_USERDATA, data)
}

#[cfg(target_arch = "x86_64")]
#[inline]
unsafe fn get_window_long(window: win32::HWND) -> win32::LONG_PTR {
    win32::GetWindowLongPtrW(window, win32::GWLP_USERDATA)
}

#[cfg(target_arch = "x86")]
#[inline]
unsafe fn set_window_long(window: win32::HWND, data: win32::LONG) -> win32::LONG {
    win32::SetWindowLongW(window, win32::GWLP_USERDATA, data)
}

#[cfg(target_arch = "x86")]
#[inline]
unsafe fn get_window_long(window: win32::HWND) -> win32::LONG {
    win32::GetWindowLongW(window, win32::GWLP_USERDATA)
}

#[cfg(target_arch = "aarch64")]
#[inline]
unsafe fn set_window_long(window: win32::HWND, data: win32::LONG_PTR) -> win32::LONG_PTR {
    win32::SetWindowLongPtrW(window, win32::GWLP_USERDATA, data)
}

#[cfg(target_arch = "aarch64")]
#[inline]
unsafe fn get_window_long(window: win32::HWND) -> win32::LONG_PTR {
    win32::GetWindowLongPtrW(window, win32::GWLP_USERDATA)
}

#[cfg(target_arch = "arm")]
#[inline]
unsafe fn set_window_long(window: win32::HWND, data: win32::LONG_PTR) -> win32::LONG_PTR {
    win32::SetWindowLongPtrW(window, win32::GWLP_USERDATA, data)
}

#[cfg(target_arch = "arm")]
#[inline]
unsafe fn get_window_long(window: win32::HWND) -> win32::LONG_PTR {
    win32::GetWindowLongPtrW(window, win32::GWLP_USERDATA)
}

unsafe extern "system" fn wnd_proc(
    window: win32::HWND,
    msg: win32::UINT,
    wparam: win32::WPARAM,
    lparam: win32::LPARAM,
) -> win32::LRESULT {
    // This make sure we actually don't do anything before the user data has been setup for the window

    let user_data = get_window_long(window);

    if user_data == 0 {
        return win32::DefWindowProcW(window, msg, wparam, lparam);
    }

    let wnd: &mut Window = std::mem::transmute(user_data);

    match msg {
        win32::WM_SYSCOMMAND => {
            if wparam == win32::SC_KEYMENU {
                return 0;
            }
        }

        win32::WM_MOUSEWHEEL => {
            let scroll = ((((wparam as u32) >> 16) & 0xffff) as i16) as f32 * 0.1;
            wnd.mouse.scroll = scroll;
        }

        win32::WM_SETCURSOR => {
            if win32::LOWORD(lparam as u32) == win32::HTCLIENT as u16 {
                win32::SetCursor(wnd.cursors[wnd.cursor as usize]);
                return 1;
            }
        }

        win32::WM_KEYDOWN => {
            update_key_state(wnd, (lparam as u32) >> 16, true);
            return 0;
        }

        win32::WM_SYSKEYDOWN => {
            update_key_state(wnd, (lparam as u32) >> 16, true);
            return 0;
        }

        win32::WM_CHAR => {
            char_down(wnd, wparam as u32);
        }

        win32::WM_SYSCHAR => {
            char_down(wnd, wparam as u32);
        }

        win32::WM_LBUTTONDOWN => wnd.mouse.state[0] = true,
        win32::WM_LBUTTONUP => wnd.mouse.state[0] = false,

        win32::WM_MOUSEMOVE => {
            let button_checks = [win32::MK_LBUTTON, win32::MK_MBUTTON, win32::MK_RBUTTON];

            for (i, button) in button_checks.iter().enumerate() {
                wnd.mouse.state[i] = (wparam & *button) == *button;
            }
        }

        win32::WM_MBUTTONDOWN => wnd.mouse.state[1] = true,
        win32::WM_MBUTTONUP => wnd.mouse.state[1] = false,
        win32::WM_RBUTTONDOWN => wnd.mouse.state[2] = true,
        win32::WM_RBUTTONUP => wnd.mouse.state[2] = false,

        win32::WM_CLOSE => {
            wnd.is_open = false;
        }

        win32::WM_KEYUP => {
            update_key_state(wnd, (lparam as u32) >> 16, false);
            return 0;
        }

        win32::WM_SYSKEYUP => {
            update_key_state(wnd, (lparam as u32) >> 16, false);
            return 0;
        }

        win32::WM_SIZE => {
            let width = (lparam as u32) & 0xffff;
            let height = ((lparam as u32) >> 16) & 0xffff;
            wnd.width = width as i32;
            wnd.height = height as i32;
        }

        win32::WM_PAINT => {
            // if we have nothing to draw here we return the default function
            if wnd.draw_params.buffer.is_null() {
                return win32::DefWindowProcW(window, msg, wparam, lparam);
            }

            let mut bitmap_info: win32::BITMAPINFO = std::mem::zeroed();

            bitmap_info.bmiHeader.biSize = std::mem::size_of::<win32::BITMAPINFOHEADER>() as u32;
            bitmap_info.bmiHeader.biPlanes = 1;
            bitmap_info.bmiHeader.biBitCount = 32;
            bitmap_info.bmiHeader.biCompression = win32::BI_BITFIELDS;
            bitmap_info.bmiHeader.biWidth = wnd.draw_params.buffer_width as i32;
            bitmap_info.bmiHeader.biHeight = -(wnd.draw_params.buffer_height as i32);
            bitmap_info.bmiColors[0].rgbRed = 0xff;
            bitmap_info.bmiColors[1].rgbGreen = 0xff;
            bitmap_info.bmiColors[2].rgbBlue = 0xff;

            let buffer_width = wnd.draw_params.buffer_width as i32;
            let buffer_height = wnd.draw_params.buffer_height as i32;
            let window_width = wnd.width as i32;
            let window_height = wnd.height as i32;

            let mut new_height = window_height;
            let mut new_width = window_width;
            let mut x_offset = 0;
            let mut y_offset = 0;

            let dc = wnd.dc;
            win32::SelectObject(dc, wnd.clear_brush as *mut c_void);

            match wnd.draw_params.scale_mode {
                ScaleMode::AspectRatioStretch => {
                    let buffer_aspect = buffer_width as f32 / buffer_height as f32;
                    let win_aspect = window_width as f32 / window_height as f32;

                    if buffer_aspect > win_aspect {
                        new_height = (window_width as f32 / buffer_aspect) as i32;
                        y_offset = (new_height - window_height) / -2;

                        if y_offset != 0 {
                            win32::Rectangle(dc, 0, 0, window_width, y_offset);
                            win32::Rectangle(
                                dc,
                                0,
                                y_offset + new_height,
                                window_width,
                                window_height,
                            );
                        }
                    } else {
                        new_width = (window_height as f32 * buffer_aspect) as i32;
                        x_offset = (new_width - window_width) / -2;

                        if x_offset != 0 {
                            win32::Rectangle(dc, 0, 0, x_offset, window_height);
                            win32::Rectangle(
                                dc,
                                x_offset + new_width,
                                0,
                                window_width,
                                window_height,
                            );
                        }
                    }
                }

                ScaleMode::Center => {
                    new_width = buffer_width;
                    new_height = buffer_height;

                    if buffer_height > window_height {
                        y_offset = -(buffer_height - window_height) / 2;
                    } else {
                        y_offset = (window_height - buffer_height) / 2;
                    }

                    if buffer_width > window_width {
                        x_offset = -(buffer_width - window_width) / 2;
                    } else {
                        x_offset = (window_width - buffer_width) / 2;
                    }

                    if y_offset > 0 {
                        win32::Rectangle(dc, 0, 0, window_width, y_offset);
                        win32::Rectangle(dc, 0, y_offset + new_height, window_width, window_height);
                    }

                    if x_offset > 0 {
                        win32::Rectangle(dc, 0, y_offset, x_offset, buffer_height + y_offset);
                        win32::Rectangle(
                            dc,
                            x_offset + buffer_width,
                            y_offset,
                            window_width,
                            buffer_height + y_offset,
                        );
                    }
                }

                ScaleMode::UpperLeft => {
                    new_width = buffer_width;
                    new_height = buffer_height;

                    if buffer_width < window_width {
                        win32::Rectangle(dc, buffer_width, 0, window_width, window_height);
                    }

                    if buffer_height < window_height {
                        win32::Rectangle(dc, 0, buffer_height, window_width, window_height);
                    }
                }

                _ => (),
            }

            win32::StretchDIBits(
                dc,
                x_offset,
                y_offset,
                new_width,
                new_height,
                0,
                0,
                wnd.draw_params.buffer_width as i32,
                wnd.draw_params.buffer_height as i32,
                std::mem::transmute(wnd.draw_params.buffer),
                &bitmap_info as *const _,
                win32::DIB_RGB_COLORS,
                win32::SRCCOPY,
            );

            win32::ValidateRect(window, std::ptr::null_mut());

            return 0;
        }

        _ => (),
    }

    win32::DefWindowProcW(window, msg, wparam, lparam)
}

#[inline]
fn to_wstring(str: &str) -> Vec<u16> {
    OsStr::new(str)
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<u16>>()
}

#[derive(Default)]
struct MouseData {
    pub x: f32,
    pub y: f32,
    pub state: [bool; 8],
    pub scroll: f32,
}

struct DrawParameters {
    buffer: *const u32,
    buffer_width: u32,
    buffer_height: u32,
    scale_mode: ScaleMode,
}

impl Default for DrawParameters {
    fn default() -> Self {
        DrawParameters {
            buffer: std::ptr::null(),
            buffer_width: 0,
            buffer_height: 0,
            scale_mode: ScaleMode::Stretch,
        }
    }
}

#[repr(C)]
pub struct Window {
    window: Option<win32::HWND>,
    dc: win32::HDC,
    clear_brush: win32::HBRUSH,
    is_open: bool,
    scale_factor: i32,
    width: i32,
    height: i32,
    key_handler: KeyHandler,
    update_rate: UpdateRate,
    cursor: CursorStyle,
    cursors: [win32::HCURSOR; 8],
    draw_params: DrawParameters,
    mouse: MouseData,
}

impl Window {
    fn open_window(
        name: &str,
        width: usize,
        height: usize,
        opts: WindowOptions,
        scale_factor: i32,
    ) -> Option<win32::HWND> {
        unsafe {
            let class_name = to_wstring("minifb_window");
            let class = win32::WNDCLASSW {
                style: win32::CS_HREDRAW | win32::CS_VREDRAW | win32::CS_OWNDC,
                lpfnWndProc: Some(wnd_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: win32::GetModuleHandleA(std::ptr::null()),
                hIcon: std::ptr::null_mut(),
                hCursor: win32::LoadCursorW(std::ptr::null_mut(), win32::IDC_ARROW),
                hbrBackground: std::ptr::null_mut(),
                lpszMenuName: std::ptr::null(),
                lpszClassName: class_name.as_ptr(),
            };

            if win32::RegisterClassW(&class) == 0 {
                // ignore the "Class already exists" error for multiple windows
                if win32::GetLastError() as u32 != 1410 {
                    println!(
                        "Unable to register class, error {}",
                        win32::GetLastError() as u32
                    );
                    return None;
                }
            }

            let window_name = to_wstring(name);

            let mut flags = 0;

            if opts.title {
                flags |= win32::WS_OVERLAPPEDWINDOW;
            }

            if opts.resize {
                flags |= win32::WS_THICKFRAME | win32::WS_MAXIMIZEBOX;
            } else {
                flags &= !win32::WS_MAXIMIZEBOX;
                flags &= !win32::WS_THICKFRAME;
            }

            if opts.borderless {
                flags &= !win32::WS_THICKFRAME;
            }

            //TODO: UpdateLayeredWindow, etc.
            //https://gist.github.com/texus/31676aba4ca774b1298e1e15133b8141
            if opts.transparency {
                flags &= win32::WS_EX_LAYERED;
            }

            if opts.none {
                flags = win32::WS_VISIBLE | win32::WS_POPUP;
            }

            let new_width = width * scale_factor as usize;
            let new_height = height * scale_factor as usize;

            let mut rect = win32::RECT {
                left: 0,
                right: new_width as win32::LONG,
                top: 0,
                bottom: new_height as win32::LONG,
            };

            win32::AdjustWindowRect(&mut rect, flags, 0);

            rect.right -= rect.left;
            rect.bottom -= rect.top;

            let handle = win32::CreateWindowExW(
                0,
                class_name.as_ptr(),
                window_name.as_ptr(),
                flags,
                win32::CW_USEDEFAULT,
                win32::CW_USEDEFAULT,
                rect.right,
                rect.bottom,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            if handle.is_null() {
                println!(
                    "Unable to create window, error {}",
                    win32::GetLastError() as u32
                );
                return None;
            }

            win32::ShowWindow(handle, win32::SW_NORMAL);

            Some(handle)
        }
    }

    pub fn new(name: &str, width: usize, height: usize, opts: WindowOptions) -> Result<Window> {
        unsafe {
            let scale_factor = Self::get_scale_factor(width, height, opts.scale);

            let handle = Self::open_window(name, width, height, opts, scale_factor);

            if handle.is_none() {
                return Err(Error::WindowCreate("Unable to create Window".to_owned()));
            }

            let window = Window {
                mouse: MouseData::default(),
                dc: win32::GetDC(handle.unwrap()),
                window: Some(handle.unwrap()),
                key_handler: KeyHandler::new(),
                update_rate: UpdateRate::new(),
                is_open: true,
                scale_factor,
                width: (width * scale_factor as usize) as i32,
                height: (height * scale_factor as usize) as i32,
                cursor: CursorStyle::Arrow,
                clear_brush: win32::CreateSolidBrush(0),
                cursors: [
                    win32::LoadCursorW(std::ptr::null_mut(), win32::IDC_ARROW),
                    win32::LoadCursorW(std::ptr::null_mut(), win32::IDC_IBEAM),
                    win32::LoadCursorW(std::ptr::null_mut(), win32::IDC_CROSS),
                    win32::LoadCursorW(std::ptr::null_mut(), win32::IDC_HAND),
                    win32::LoadCursorW(std::ptr::null_mut(), win32::IDC_HAND),
                    win32::LoadCursorW(std::ptr::null_mut(), win32::IDC_SIZEWE),
                    win32::LoadCursorW(std::ptr::null_mut(), win32::IDC_SIZENS),
                    win32::LoadCursorW(std::ptr::null_mut(), win32::IDC_SIZEALL),
                ],
                draw_params: DrawParameters {
                    scale_mode: opts.scale_mode,
                    ..DrawParameters::default()
                },
            };

            if opts.topmost {
                window.topmost(true)
            }

            Ok(window)
        }
    }

    #[inline]
    pub fn set_title(&mut self, title: &str) {
        unsafe {
            let title_name = to_wstring(title);
            win32::SetWindowTextW(self.window.unwrap(), title_name.as_ptr());
        }
    }

    #[inline]
    pub fn set_icon(&mut self, icon: Icon) {
        unsafe {
            if let Icon::Path(s_pointer) = icon {
                let mut buffer: Vec<u16> = Vec::new();

                // call once to get the size of the buffer
                let return_value = win32::GetFullPathNameW(
                    s_pointer,
                    0,
                    buffer.as_mut_ptr(),
                    std::ptr::null_mut(),
                );

                // adjust size of the buffer
                buffer.reserve(return_value as usize);

                let _ = win32::GetFullPathNameW(
                    s_pointer,
                    return_value,
                    buffer.as_mut_ptr(),
                    std::ptr::null_mut(),
                );

                let path = buffer.as_ptr();

                // cx and cy are 0 so Windows uses the size of the resource
                let icon = win32::LoadImageW(
                    std::ptr::null_mut(),
                    path,
                    win32::IMAGE_ICON,
                    0,
                    0,
                    win32::LR_DEFAULTSIZE | win32::LR_LOADFROMFILE,
                );

                if let Some(handle) = self.window {
                    win32::SendMessageW(
                        handle,
                        win32::WM_SETICON,
                        win32::ICON_SMALL as win32::WPARAM,
                        icon as win32::LPARAM,
                    );

                    win32::SendMessageW(
                        handle,
                        win32::WM_SETICON,
                        win32::ICON_BIG as win32::WPARAM,
                        icon as win32::LPARAM,
                    );
                }
            }
        }
    }

    #[inline]
    pub fn get_window_handle(&self) -> *mut c_void {
        self.window.unwrap() as *mut c_void
    }

    #[inline]
    pub fn set_position(&mut self, x: isize, y: isize) {
        unsafe {
            win32::SetWindowPos(
                self.window.unwrap(),
                std::ptr::null_mut(),
                x as i32,
                y as i32,
                0,
                0,
                win32::SWP_SHOWWINDOW | win32::SWP_NOSIZE,
            );
        }
    }

    #[inline]
    pub fn get_position(&self) -> (isize, isize) {
        let (mut x, mut y) = (0, 0);

        unsafe {
            let mut rect = win32::RECT {
                left: 0,
                right: 0,
                top: 0,
                bottom: 0,
            };
            if win32::GetWindowRect(self.window.unwrap(), &mut rect) != 0 {
                x = rect.left;
                y = rect.top;
            }
        }
        (x as isize, y as isize)
    }

    #[inline]
    pub fn topmost(&self, topmost: bool) {
        unsafe {
            win32::SetWindowPos(
                self.window.unwrap(),
                if topmost {
                    win32::HWND_TOPMOST
                } else {
                    win32::HWND_TOP
                },
                0,
                0,
                0,
                0,
                win32::SWP_SHOWWINDOW | win32::SWP_NOSIZE | win32::SWP_NOMOVE,
            )
        };
    }

    #[inline]
    pub fn get_size(&self) -> (usize, usize) {
        (self.width as usize, self.height as usize)
    }

    #[inline]
    pub fn get_mouse_pos(&self, mode: MouseMode) -> Option<(f32, f32)> {
        let s = self.scale_factor as f32;
        let w = self.width as f32;
        let h = self.height as f32;

        // TODO: Needs to be fixed with resize support
        mode.get_pos(self.mouse.x, self.mouse.y, s, w, h)
    }

    #[inline]
    pub fn get_unscaled_mouse_pos(&self, mode: MouseMode) -> Option<(f32, f32)> {
        let w = self.width as f32;
        let h = self.height as f32;

        // TODO: Needs to be fixed with resize support
        mode.get_pos(self.mouse.x, self.mouse.y, 1.0, w, h)
    }

    #[inline]
    pub fn get_mouse_down(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.mouse.state[0],
            MouseButton::Middle => self.mouse.state[1],
            MouseButton::Right => self.mouse.state[2],
        }
    }

    #[inline]
    pub fn get_scroll_wheel(&self) -> Option<(f32, f32)> {
        if self.mouse.scroll.abs() > 0.0 {
            Some((0.0, self.mouse.scroll))
        } else {
            None
        }
    }

    #[inline]
    pub fn set_cursor_style(&mut self, cursor: CursorStyle) {
        self.cursor = cursor;
    }

    #[inline]
    pub fn set_rate(&mut self, rate: Option<Duration>) {
        self.update_rate.set_rate(rate);
    }

    #[inline]
    pub fn update_rate(&mut self) {
        self.update_rate.update();
    }

    #[inline]
    pub fn get_keys(&self) -> Vec<Key> {
        self.key_handler.get_keys()
    }

    #[inline]
    pub fn get_keys_pressed(&self, repeat: KeyRepeat) -> Vec<Key> {
        self.key_handler.get_keys_pressed(repeat)
    }

    #[inline]
    pub fn get_keys_released(&self) -> Vec<Key> {
        self.key_handler.get_keys_released()
    }

    #[inline]
    pub fn is_key_down(&self, key: Key) -> bool {
        self.key_handler.is_key_down(key)
    }

    #[inline]
    pub fn set_input_callback(&mut self, callback: Box<dyn InputCallback>) {
        self.key_handler.set_input_callback(callback)
    }

    #[inline]
    pub fn set_key_repeat_delay(&mut self, delay: f32) {
        self.key_handler.set_key_repeat_delay(delay)
    }

    #[inline]
    pub fn set_key_repeat_rate(&mut self, rate: f32) {
        self.key_handler.set_key_repeat_rate(rate)
    }

    #[inline]
    pub fn is_key_pressed(&self, key: Key, repeat: KeyRepeat) -> bool {
        self.key_handler.is_key_pressed(key, repeat)
    }

    #[inline]
    pub fn is_key_released(&self, key: Key) -> bool {
        self.key_handler.is_key_released(key)
    }

    #[inline]
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    fn generic_update(&mut self, window: win32::HWND) {
        unsafe {
            let mut point: win32::POINT = std::mem::zeroed();

            win32::GetCursorPos(&mut point);
            win32::ScreenToClient(window, &mut point);

            self.mouse.x = point.x as f32;
            self.mouse.y = point.y as f32;
            self.mouse.scroll = 0.0;

            self.key_handler.update();

            set_window_long(window, std::mem::transmute(self));
        }
    }

    fn message_loop(&self, _window: win32::HWND) {
        unsafe {
            let mut msg = std::mem::zeroed();

            while win32::PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, win32::PM_REMOVE) != 0 {
                win32::TranslateMessage(&msg);
                win32::DispatchMessageW(&msg);
            }
        }
    }

    #[allow(clippy::identity_op)]
    pub fn set_background_color(&mut self, color: u32) {
        unsafe {
            win32::DeleteObject(self.clear_brush as *mut c_void);
            let r = (color >> 16) & 0xff;
            let g = (color >> 8) & 0xff;
            let b = (color >> 0) & 0xff;
            self.clear_brush = win32::CreateSolidBrush((b << 16) | (g << 8) | r);
        }
    }

    #[inline]
    pub fn set_cursor_visibility(&mut self, visibility: bool) {
        unsafe {
            win32::ShowCursor(visibility as i32);
        }
    }

    pub fn update_with_buffer_stride(
        &mut self,
        buffer: &[u32],
        buf_width: usize,
        buf_height: usize,
        buf_stride: usize,
    ) -> Result<()> {
        let window = self.window.unwrap();

        self.generic_update(window);

        check_buffer_size(buffer, buf_width, buf_height, buf_stride)?;

        self.draw_params.buffer = buffer.as_ptr();
        self.draw_params.buffer_width = buf_width as u32;
        self.draw_params.buffer_height = buf_height as u32;

        unsafe {
            win32::InvalidateRect(window, std::ptr::null_mut(), win32::TRUE);
        }

        self.message_loop(window);

        Ok(())
    }

    #[inline]
    pub fn update(&mut self) {
        let window = self.window.unwrap();

        self.generic_update(window);
        self.message_loop(window);
    }

    #[inline]
    pub fn is_active(&mut self) -> bool {
        match self.window {
            Some(hwnd) => {
                let active = unsafe { win32::GetActiveWindow() };
                !active.is_null() && active == hwnd
            }
            None => false,
        }
    }

    unsafe fn get_scale_factor(width: usize, height: usize, scale: Scale) -> i32 {
        let factor: i32 = match scale {
            Scale::X1 => 1,
            Scale::X2 => 2,
            Scale::X4 => 4,
            Scale::X8 => 8,
            Scale::X16 => 16,
            Scale::X32 => 32,
            Scale::FitScreen => {
                let screen_x = win32::GetSystemMetrics(win32::SM_CXSCREEN) as i32;
                let screen_y = win32::GetSystemMetrics(win32::SM_CYSCREEN) as i32;

                let mut scale = 1i32;

                loop {
                    let w = width as i32 * (scale + 1);
                    let h = height as i32 * (scale + 1);

                    if w > screen_x || h > screen_y {
                        break;
                    }

                    scale *= 2;
                }

                scale
            }
        };

        factor
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            win32::ReleaseDC(self.window.unwrap(), self.dc);

            if self.window.is_some() {
                win32::DestroyWindow(self.window.unwrap());
            }
        }
    }
}
