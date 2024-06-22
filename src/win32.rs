#![allow(non_snake_case, non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]

use std::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void};

pub type BOOL = c_int;
pub type BYTE = c_uchar;
pub type CHAR = c_char;
pub type DWORD = c_ulong;
pub type LONG = c_long;
pub type LONG_PTR = isize;
pub type UINT = c_uint;
pub type UINT_PTR = usize;
pub type VOID = c_void;
pub type WCHAR = u16;
pub type WORD = c_ushort;

pub enum HBRUSH__ {}
pub enum HDC__ {}
pub enum HICON__ {}
pub enum HINSTANCE__ {}
pub enum HMENU__ {}
pub enum HWND__ {}

pub type ATOM = WORD;
pub type COLORREF = DWORD;
pub type HANDLE = *mut c_void;
pub type HBRUSH = *mut HBRUSH__;
pub type HCURSOR = HICON;
pub type HDC = *mut HDC__;
pub type HGDIOBJ = *mut c_void;
pub type HICON = *mut HICON__;
pub type HINSTANCE = *mut HINSTANCE__;
pub type HMENU = *mut HMENU__;
pub type HMODULE = HINSTANCE;
pub type HWND = *mut HWND__;
pub type LPARAM = LONG_PTR;
pub type LPCSTR = *const CHAR;
pub type LPCWSTR = *const WCHAR;
pub type LPMSG = *mut MSG;
pub type LPRECT = *mut RECT;
pub type LPVOID = *mut c_void;
pub type LPWSTR = *mut WCHAR;
pub type LRESULT = LONG_PTR;
pub type LPPOINT = *mut POINT;
pub type WNDPROC = Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>;
pub type WPARAM = UINT_PTR;

#[repr(C)]
pub struct BITMAPINFOHEADER {
    pub biSize: DWORD,
    pub biWidth: LONG,
    pub biHeight: LONG,
    pub biPlanes: WORD,
    pub biBitCount: WORD,
    pub biCompression: DWORD,
    pub biSizeImage: DWORD,
    pub biXPelsPerMeter: LONG,
    pub biYPelsPerMeter: LONG,
    pub biClrUsed: DWORD,
    pub biClrImportant: DWORD,
}

#[repr(C)]
pub struct RGBQUAD {
    pub rgbBlue: BYTE,
    pub rgbGreen: BYTE,
    pub rgbRed: BYTE,
    pub rgbReserved: BYTE,
}

#[repr(C)]
pub struct BITMAPINFO {
    pub bmiHeader: BITMAPINFOHEADER,
    pub bmiColors: [RGBQUAD; 3],
}

#[repr(C)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}

#[repr(C)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}

#[repr(C)]
pub struct WNDCLASSW {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
}

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
}

pub const BI_BITFIELDS: DWORD = 3;
pub const CS_HREDRAW: UINT = 0x0002;
pub const CS_OWNDC: UINT = 0x0020;
pub const CS_VREDRAW: UINT = 0x0001;
pub const CW_USEDEFAULT: c_int = -2147483648;
pub const ICON_SMALL: UINT = 0;
pub const DIB_RGB_COLORS: DWORD = 0;
pub const WM_SETICON: UINT = 0x0080;
pub const GWLP_USERDATA: c_int = -21;
pub const HTCLIENT: LRESULT = 1;
pub const ICON_BIG: UINT = 1;
pub const IDC_ARROW: LPCWSTR = 32512 as LPCWSTR;
pub const IDC_CROSS: LPCWSTR = 32515 as LPCWSTR;
pub const IDC_HAND: LPCWSTR = 32649 as LPCWSTR;
pub const IDC_IBEAM: LPCWSTR = 32513 as LPCWSTR;
pub const IDC_SIZEALL: LPCWSTR = 32646 as LPCWSTR;
pub const IDC_SIZENS: LPCWSTR = 32645 as LPCWSTR;
pub const IDC_SIZEWE: LPCWSTR = 32644 as LPCWSTR;
pub const MK_LBUTTON: WPARAM = 0x0001;
pub const MK_MBUTTON: WPARAM = 0x0010;
pub const MK_RBUTTON: WPARAM = 0x0002;
pub const SC_KEYMENU: WPARAM = 0xF100;
pub const SRCCOPY: DWORD = 0x00CC0020;
pub const SW_NORMAL: c_int = 1;
pub const TRUE: BOOL = 1;
pub const WM_CHAR: UINT = 0x0102;
pub const WM_CLOSE: UINT = 0x0010;
pub const WM_KEYDOWN: UINT = 0x0100;
pub const WM_KEYUP: UINT = 0x0101;
pub const HWND_TOPMOST: HWND = -1isize as HWND;
pub const WM_LBUTTONDOWN: UINT = 0x0201;
pub const WM_LBUTTONUP: UINT = 0x0202;
pub const WM_MBUTTONDOWN: UINT = 0x0207;
pub const HWND_TOP: HWND = 0 as HWND;
pub const WM_MBUTTONUP: UINT = 0x0208;
pub const WM_MOUSEMOVE: UINT = 0x0200;
pub const WM_MOUSEWHEEL: UINT = 0x020A;
pub const WM_PAINT: UINT = 0x000F;
pub const WM_RBUTTONDOWN: UINT = 0x0204;
pub const SWP_NOSIZE: UINT = 0x0001;
pub const WM_RBUTTONUP: UINT = 0x0205;
pub const WM_SETCURSOR: UINT = 0x0020;
pub const WM_SIZE: UINT = 0x0005;
pub const SWP_NOMOVE: UINT = 0x0002;
pub const WM_SYSCHAR: UINT = 0x0106;
pub const PM_REMOVE: UINT = 0x0001;
pub const WM_SYSCOMMAND: UINT = 0x0112;
pub const SWP_SHOWWINDOW: UINT = 0x0040;
pub const WM_SYSKEYDOWN: UINT = 0x0104;
pub const WM_SYSKEYUP: UINT = 0x0105;
pub const WS_CAPTION: DWORD = 0x00C00000;
pub const WS_EX_LAYERED: DWORD = 0x00080000;
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;
pub const SM_CXSCREEN: c_int = 0;
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub const IMAGE_ICON: UINT = 1;
pub const WS_OVERLAPPED: DWORD = 0x00000000;
pub const LR_DEFAULTSIZE: UINT = 0x00000040;
pub const WS_POPUP: DWORD = 0x80000000;
pub const WS_SYSMENU: DWORD = 0x00080000;
pub const LR_LOADFROMFILE: UINT = 0x00000010;
pub const SM_CYSCREEN: c_int = 1;
pub const WS_THICKFRAME: DWORD = 0x00040000;
pub const WS_VISIBLE: DWORD = 0x10000000;
pub const WS_OVERLAPPEDWINDOW: DWORD =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

#[inline]
pub fn LOWORD(l: DWORD) -> WORD {
    (l & 0xffff) as WORD
}

#[link(name = "User32")]
extern "system" {
    pub fn AdjustWindowRect(lpRect: LPRECT, dwStyle: DWORD, bMenu: BOOL) -> BOOL;
    pub fn CreateSolidBrush(color: COLORREF) -> HBRUSH;
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        x: c_int,
        y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    pub fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn DeleteObject(ho: HGDIOBJ) -> BOOL;
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    pub fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT;
    pub fn GetActiveWindow() -> HWND;
    pub fn GetCursorPos(lpPoint: LPPOINT) -> BOOL;
    pub fn GetDC(hWnd: HWND) -> HDC;
    pub fn GetFullPathNameW(
        lpFileName: LPCWSTR,
        nBufferLength: DWORD,
        lpBuffer: LPWSTR,
        lpFilePart: *mut LPWSTR,
    ) -> DWORD;
    pub fn GetLastError() -> DWORD;
    pub fn GetSystemMetrics(nIndex: c_int) -> c_int;
    pub fn GetWindowRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;
    pub fn InvalidateRect(hWnd: HWND, lpRect: *const RECT, bErase: BOOL) -> BOOL;
    pub fn LoadCursorW(hInstance: HINSTANCE, lpCursorName: LPCWSTR) -> HCURSOR;
    pub fn LoadImageW(
        hInst: HINSTANCE,
        name: LPCWSTR,
        type_: UINT,
        cx: c_int,
        cy: c_int,
        fuLoad: UINT,
    ) -> HANDLE;
    pub fn PeekMessageW(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
        wRemoveMsg: UINT,
    ) -> BOOL;
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
    pub fn ReleaseDC(hWnd: HWND, hDC: HDC) -> c_int;
    pub fn ScreenToClient(hWnd: HWND, lpPoint: LPPOINT) -> BOOL;
    pub fn SelectObject(hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
    pub fn SendMessageW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn SetCursor(hCursor: HCURSOR) -> HCURSOR;
    pub fn SetWindowPos(
        hWnd: HWND,
        hWndInsertAfter: HWND,
        X: c_int,
        Y: c_int,
        cx: c_int,
        cy: c_int,
        uFlags: UINT,
    ) -> BOOL;
    pub fn SetWindowTextW(hWnd: HWND, lpString: LPCWSTR) -> BOOL;
    pub fn ShowCursor(bShow: BOOL) -> c_int;
    pub fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    pub fn TranslateMessage(lpmsg: *const MSG) -> BOOL;
    pub fn ValidateRect(hWnd: HWND, lpRect: *const RECT) -> BOOL;

    #[cfg(target_pointer_width = "32")]
    pub fn GetWindowLongW(hWnd: HWND, nIndex: c_int) -> LONG;
    #[cfg(target_pointer_width = "32")]
    pub fn SetWindowLongW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG) -> LONG;

    #[cfg(target_pointer_width = "64")]
    pub fn SetWindowLongPtrW(hWnd: HWND, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR;
    #[cfg(target_pointer_width = "64")]
    pub fn GetWindowLongPtrW(hWnd: HWND, nIndex: c_int) -> LONG_PTR;
}

#[link(name = "Gdi32")]
extern "system" {
    pub fn Rectangle(hdc: HDC, left: c_int, top: c_int, right: c_int, bottom: c_int) -> BOOL;
    pub fn StretchDIBits(
        hdc: HDC,
        XDest: c_int,
        YDest: c_int,
        nDestWidth: c_int,
        nDestHeight: c_int,
        XSrc: c_int,
        YSrc: c_int,
        nSrcWidth: c_int,
        nSrcHeight: c_int,
        lpBits: *const VOID,
        lpBitsInfo: *const BITMAPINFO,
        iUsage: UINT,
        dwRop: DWORD,
    ) -> c_int;
}

#[link(name = "Kernel32")]
extern "system" {
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
}
