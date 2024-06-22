#![allow(non_upper_case_globals, clippy::upper_case_acronyms)]

use std::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort};

pub enum _XDisplay {}
pub enum _XGC {}
pub enum _XIC {}
pub enum _XIM {}
pub enum _XrmHashBucketRec {}

pub type Atom = XID;
pub type Bool = c_int;
pub type Colormap = XID;
pub type Cursor = XID;
pub type Display = _XDisplay;
pub type Drawable = XID;
pub type Font = XID;
pub type GC = *mut _XGC;
pub type KeyCode = c_uchar;
pub type KeySym = XID;
pub type Pixmap = XID;
pub type Status = Bool;
pub type Time = c_ulong;
pub type VisualID = XID;
pub type Window = XID;
pub type XContext = c_int;
pub type XIC = *mut _XIC;
pub type XID = c_ulong;
pub type XIM = *mut _XIM;
pub type XPointer = *mut c_char;
pub type XrmDatabase = *mut _XrmHashBucketRec;

#[repr(C)]
pub struct XExtData {
    pub number: c_int,
    pub next: *mut XExtData,
    pub free_private: Option<unsafe extern "C" fn() -> c_int>,
    pub private_data: XPointer,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Visual {
    pub ext_data: *mut XExtData,
    pub visualid: VisualID,
    pub class: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub bits_per_rgb: c_int,
    pub map_entries: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XButtonEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub button: c_uint,
    pub same_screen: Bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XKeyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub keycode: c_uint,
    pub same_screen: Bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XConfigureEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub above: Window,
    pub override_redirect: Bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XAnyEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ClientMessageData {
    longs: [c_long; 5],
}

impl ClientMessageData {
    pub fn get_long(&self, index: usize) -> c_long {
        self.longs[index]
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XClientMessageEvent {
    pub type_: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: c_int,
    pub data: ClientMessageData,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union XEvent {
    pub type_: c_int,
    pub any: XAnyEvent,
    pub client_message: XClientMessageEvent,
    pub configure: XConfigureEvent,
    pub button: XButtonEvent,
    pub key: XKeyEvent,
    pub pad: [c_long; 24],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XWindowAttributes {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub depth: c_int,
    pub visual: *mut Visual,
    pub root: Window,
    pub class: c_int,
    pub bit_gravity: c_int,
    pub win_gravity: c_int,
    pub backing_store: c_int,
    pub backing_planes: c_ulong,
    pub backing_pixel: c_ulong,
    pub save_under: Bool,
    pub colormap: Colormap,
    pub map_installed: Bool,
    pub map_state: c_int,
    pub all_event_masks: c_long,
    pub your_event_mask: c_long,
    pub do_not_propagate_mask: c_long,
    pub override_redirect: Bool,
    pub screen: *mut Screen,
}

#[repr(C)]
pub struct Depth {
    pub depth: c_int,
    pub nvisuals: c_int,
    pub visuals: *mut Visual,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Screen {
    pub ext_data: *mut XExtData,
    pub display: *mut Display,
    pub root: Window,
    pub width: c_int,
    pub height: c_int,
    pub mwidth: c_int,
    pub mheight: c_int,
    pub ndepths: c_int,
    pub depths: *mut Depth,
    pub root_depth: c_int,
    pub root_visual: *mut Visual,
    pub default_gc: GC,
    pub cmap: Colormap,
    pub white_pixel: c_ulong,
    pub black_pixel: c_ulong,
    pub max_maps: c_int,
    pub min_maps: c_int,
    pub backing_store: c_int,
    pub save_unders: Bool,
    pub root_input_mask: c_long,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XVisualInfo {
    pub visual: *mut Visual,
    pub visualid: VisualID,
    pub screen: c_int,
    pub depth: c_int,
    pub class: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub colormap_size: c_int,
    pub bits_per_rgb: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XPixmapFormatValues {
    pub depth: c_int,
    pub bits_per_pixel: c_int,
    pub scanline_pad: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ImageFns {
    pub create_image: Option<
        unsafe extern "C" fn(
            _: *mut Display,
            _: *mut Visual,
            _: c_uint,
            _: c_int,
            _: c_int,
            _: *mut c_char,
            _: c_uint,
            _: c_uint,
            _: c_int,
            _: c_int,
        ) -> *mut XImage,
    >,
    pub destroy_image: Option<unsafe extern "C" fn(_: *mut XImage) -> c_int>,
    pub get_pixel: Option<unsafe extern "C" fn(_: *mut XImage, _: c_int, _: c_int) -> c_ulong>,
    pub put_pixel:
        Option<unsafe extern "C" fn(_: *mut XImage, _: c_int, _: c_int, _: c_ulong) -> c_int>,
    pub sub_image: Option<
        unsafe extern "C" fn(
            _: *mut XImage,
            _: c_int,
            _: c_int,
            _: c_uint,
            _: c_uint,
        ) -> *mut XImage,
    >,
    pub add_pixel: Option<unsafe extern "C" fn(_: *mut XImage, _: c_long) -> c_int>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XImage {
    pub width: c_int,
    pub height: c_int,
    pub xoffset: c_int,
    pub format: c_int,
    pub data: *mut c_char,
    pub byte_order: c_int,
    pub bitmap_unit: c_int,
    pub bitmap_bit_order: c_int,
    pub bitmap_pad: c_int,
    pub depth: c_int,
    pub bytes_per_line: c_int,
    pub bits_per_pixel: c_int,
    pub red_mask: c_ulong,
    pub green_mask: c_ulong,
    pub blue_mask: c_ulong,
    pub obdata: XPointer,
    pub funcs: ImageFns,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XSetWindowAttributes {
    pub background_pixmap: Pixmap,
    pub background_pixel: c_ulong,
    pub border_pixmap: Pixmap,
    pub border_pixel: c_ulong,
    pub bit_gravity: c_int,
    pub win_gravity: c_int,
    pub backing_store: c_int,
    pub backing_planes: c_ulong,
    pub backing_pixel: c_ulong,
    pub save_under: Bool,
    pub event_mask: c_long,
    pub do_not_propagate_mask: c_long,
    pub override_redirect: Bool,
    pub colormap: Colormap,
    pub cursor: Cursor,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XGCValues {
    pub function: c_int,
    pub plane_mask: c_ulong,
    pub foreground: c_ulong,
    pub background: c_ulong,
    pub line_width: c_int,
    pub line_style: c_int,
    pub cap_style: c_int,
    pub join_style: c_int,
    pub fill_style: c_int,
    pub fill_rule: c_int,
    pub arc_mode: c_int,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub ts_x_origin: c_int,
    pub ts_y_origin: c_int,
    pub font: Font,
    pub subwindow_mode: c_int,
    pub graphics_exposures: Bool,
    pub clip_x_origin: c_int,
    pub clip_y_origin: c_int,
    pub clip_mask: Pixmap,
    pub dash_offset: c_int,
    pub dashes: c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AspectRatio {
    pub x: c_int,
    pub y: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XSizeHints {
    pub flags: c_long,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub min_width: c_int,
    pub min_height: c_int,
    pub max_width: c_int,
    pub max_height: c_int,
    pub width_inc: c_int,
    pub height_inc: c_int,
    pub min_aspect: AspectRatio,
    pub max_aspect: AspectRatio,
    pub base_width: c_int,
    pub base_height: c_int,
    pub win_gravity: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct XColor {
    pub pixel: c_ulong,
    pub red: c_ushort,
    pub green: c_ushort,
    pub blue: c_ushort,
    pub flags: c_char,
    pub pad: c_char,
}

pub const AllocNone: c_int = 0;
pub const Button1: c_uint = 1;
pub const Button2: c_uint = 2;
pub const Button3: c_uint = 3;
pub const Button4: c_uint = 4;
pub const Button5: c_uint = 5;
pub const ButtonPress: c_int = 4;
pub const ButtonPressMask: c_long = 0x0000_0004;
pub const ButtonRelease: c_int = 5;
pub const ButtonReleaseMask: c_long = 0x0000_0008;
pub const ClientMessage: c_int = 33;
pub const ConfigureNotify: c_int = 22;
pub const CWBackingStore: c_ulong = 0x0040;
pub const CWBackPixel: c_ulong = 0x0002;
pub const CWBorderPixel: c_ulong = 0x0008;
pub const CWColormap: c_ulong = 0x2000;
pub const False: Bool = 0;
pub const FocusChangeMask: c_long = 0x0020_0000;
pub const FocusIn: c_int = 9;
pub const FocusOut: c_int = 10;
pub const InputOutput: c_int = 1;
pub const KeyPress: c_int = 2;
pub const KeyPressMask: c_long = 0x0000_0001;
pub const KeyRelease: c_int = 3;
pub const KeyReleaseMask: c_long = 0x0000_0002;
pub const NoSymbol: c_int = 0;
pub const NotUseful: c_int = 0;
pub const PMaxSize: c_long = 0x0020;
pub const PMinSize: c_long = 0x0010;
pub const PropModeReplace: c_int = 0;
pub const StructureNotifyMask: c_long = 0x0002_0000;
pub const True: Bool = 1;
pub const TrueColor: c_int = 4;
pub const XIMPreeditNothing: c_int = 0x0008;
pub const XIMStatusNothing: c_int = 0x0400;
pub const XK_0: c_uint = 0x030;
pub const XK_1: c_uint = 0x031;
pub const XK_2: c_uint = 0x032;
pub const XK_3: c_uint = 0x033;
pub const XK_4: c_uint = 0x034;
pub const XK_5: c_uint = 0x035;
pub const XK_6: c_uint = 0x036;
pub const XK_7: c_uint = 0x037;
pub const XK_8: c_uint = 0x038;
pub const XK_9: c_uint = 0x039;
pub const XK_a: c_uint = 0x061;
pub const XK_Alt_L: c_uint = 0xFFE9;
pub const XK_Alt_R: c_uint = 0xFFEA;
pub const XK_apostrophe: c_uint = 0x027;
pub const XK_b: c_uint = 0x062;
pub const XK_backslash: c_uint = 0x05c;
pub const XK_BackSpace: c_uint = 0xFF08;
pub const XK_bracketleft: c_uint = 0x05b;
pub const XK_bracketright: c_uint = 0x05d;
pub const XK_c: c_uint = 0x063;
pub const XK_Caps_Lock: c_uint = 0xFFE5;
pub const XK_comma: c_uint = 0x02c;
pub const XK_Control_L: c_uint = 0xFFE3;
pub const XK_Control_R: c_uint = 0xFFE4;
pub const XK_d: c_uint = 0x064;
pub const XK_Delete: c_uint = 0xFFFF;
pub const XK_Down: c_uint = 0xFF54;
pub const XK_e: c_uint = 0x065;
pub const XK_End: c_uint = 0xFF57;
pub const XK_equal: c_uint = 0x03d;
pub const XK_Escape: c_uint = 0xFF1B;
pub const XK_f: c_uint = 0x066;
pub const XK_F1: c_uint = 0xFFBE;
pub const XK_F10: c_uint = 0xFFC7;
pub const XK_F11: c_uint = 0xFFC8;
pub const XK_F12: c_uint = 0xFFC9;
pub const XK_F2: c_uint = 0xFFBF;
pub const XK_F3: c_uint = 0xFFC0;
pub const XK_F4: c_uint = 0xFFC1;
pub const XK_F5: c_uint = 0xFFC2;
pub const XK_F6: c_uint = 0xFFC3;
pub const XK_F7: c_uint = 0xFFC4;
pub const XK_F8: c_uint = 0xFFC5;
pub const XK_F9: c_uint = 0xFFC6;
pub const XK_g: c_uint = 0x067;
pub const XK_grave: c_uint = 0x060;
pub const XK_h: c_uint = 0x068;
pub const XK_Home: c_uint = 0xFF50;
pub const XK_i: c_uint = 0x069;
pub const XK_Insert: c_uint = 0xFF63;
pub const XK_j: c_uint = 0x06a;
pub const XK_k: c_uint = 0x06b;
pub const XK_KP_0: c_uint = 0xFFB0;
pub const XK_KP_1: c_uint = 0xFFB1;
pub const XK_KP_2: c_uint = 0xFFB2;
pub const XK_KP_3: c_uint = 0xFFB3;
pub const XK_KP_4: c_uint = 0xFFB4;
pub const XK_KP_5: c_uint = 0xFFB5;
pub const XK_KP_6: c_uint = 0xFFB6;
pub const XK_KP_7: c_uint = 0xFFB7;
pub const XK_KP_8: c_uint = 0xFFB8;
pub const XK_KP_9: c_uint = 0xFFB9;
pub const XK_KP_Add: c_uint = 0xFFAB;
pub const XK_KP_Decimal: c_uint = 0xFFAE;
pub const XK_KP_Divide: c_uint = 0xFFAF;
pub const XK_KP_Enter: c_uint = 0xFF8D;
pub const XK_KP_Equal: c_uint = 0xFFBD;
pub const XK_KP_Multiply: c_uint = 0xFFAA;
pub const XK_KP_Separator: c_uint = 0xFFAC;
pub const XK_KP_Subtract: c_uint = 0xFFAD;
pub const XK_l: c_uint = 0x06c;
pub const XK_Left: c_uint = 0xFF51;
pub const XK_m: c_uint = 0x06d;
pub const XK_Menu: c_uint = 0xFF67;
pub const XK_minus: c_uint = 0x02d;
pub const XK_n: c_uint = 0x06e;
pub const XK_Num_Lock: c_uint = 0xFF7F;
pub const XK_o: c_uint = 0x06f;
pub const XK_p: c_uint = 0x070;
pub const XK_Page_Down: c_uint = 0xFF56;
pub const XK_Page_Up: c_uint = 0xFF55;
pub const XK_Pause: c_uint = 0xFF13;
pub const XK_period: c_uint = 0x02e;
pub const XK_q: c_uint = 0x071;
pub const XK_r: c_uint = 0x072;
pub const XK_Return: c_uint = 0xFF0D;
pub const XK_Right: c_uint = 0xFF53;
pub const XK_s: c_uint = 0x073;
pub const XK_Scroll_Lock: c_uint = 0xFF14;
pub const XK_semicolon: c_uint = 0x03b;
pub const XK_Shift_L: c_uint = 0xFFE1;
pub const XK_Shift_R: c_uint = 0xFFE2;
pub const XK_slash: c_uint = 0x02f;
pub const XK_space: c_uint = 0x020;
pub const XK_Super_L: c_uint = 0xFFEB;
pub const XK_Super_R: c_uint = 0xFFEC;
pub const XK_t: c_uint = 0x074;
pub const XK_Tab: c_uint = 0xFF09;
pub const XK_u: c_uint = 0x075;
pub const XK_Up: c_uint = 0xFF52;
pub const XK_v: c_uint = 0x076;
pub const XK_w: c_uint = 0x077;
pub const XK_x: c_uint = 0x078;
pub const XK_y: c_uint = 0x079;
pub const XK_z: c_uint = 0x07a;
pub const XNClientWindow_0: &[u8] = b"clientWindow\0";
pub const XNFocusWindow_0: &[u8] = b"focusWindow\0";
pub const XNInputStyle_0: &[u8] = b"inputStyle\0";
pub const ZPixmap: c_int = 2;

#[link(name = "X11")]
extern "C" {
    pub fn XInitThreads() -> c_int;
    pub fn XOpenDisplay(_1: *const c_char) -> *mut Display;
    pub fn XkbSetDetectableAutoRepeat(_3: *mut Display, _2: c_int, _1: *mut c_int) -> c_int;
    pub fn XMatchVisualInfo(
        _5: *mut Display,
        _4: c_int,
        _3: c_int,
        _2: c_int,
        _1: *mut XVisualInfo,
    ) -> c_int;
    pub fn XDefaultScreen(_1: *mut Display) -> c_int;
    pub fn XDefaultVisual(_2: *mut Display, _1: c_int) -> *mut Visual;
    pub fn XDefaultDepth(_2: *mut Display, _1: c_int) -> c_int;
    pub fn XDefaultGC(_2: *mut Display, _1: c_int) -> GC;
    pub fn XDisplayWidth(_2: *mut Display, _1: c_int) -> c_int;
    pub fn XDisplayHeight(_2: *mut Display, _1: c_int) -> c_int;
    pub fn XrmUniqueQuark() -> c_int;
    pub fn XListPixmapFormats(_2: *mut Display, _1: *mut c_int) -> *mut XPixmapFormatValues;
    pub fn XkbQueryExtension(
        _6: *mut Display,
        _5: *mut c_int,
        _4: *mut c_int,
        _3: *mut c_int,
        _2: *mut c_int,
        _1: *mut c_int,
    ) -> c_int;
    pub fn XInternAtom(_3: *mut Display, _2: *const c_char, _1: c_int) -> c_ulong;
    pub fn XCloseDisplay(_1: *mut Display) -> c_int;
    pub fn XDefaultRootWindow(_1: *mut Display) -> c_ulong;
    pub fn XBlackPixel(_2: *mut Display, _1: c_int) -> c_ulong;
    pub fn XCreateColormap(_4: *mut Display, _3: c_ulong, _2: *mut Visual, _1: c_int) -> c_ulong;
    pub fn XCreateWindow(
        _12: *mut Display,
        _11: c_ulong,
        _10: c_int,
        _9: c_int,
        _8: c_uint,
        _7: c_uint,
        _6: c_uint,
        _5: c_int,
        _4: c_uint,
        _3: *mut Visual,
        _2: c_ulong,
        _1: *mut XSetWindowAttributes,
    ) -> c_ulong;
    pub fn XSetLocaleModifiers(_1: *const c_char) -> *mut c_char;
    pub fn XOpenIM(_4: *mut Display, _3: XrmDatabase, _2: *mut c_char, _1: *mut c_char) -> XIM;
    pub fn XCreateIC(_1: XIM, ...) -> XIC;
    pub fn XSetICFocus(_1: XIC);
    pub fn XSelectInput(_3: *mut Display, _2: c_ulong, _1: c_long) -> c_int;
    pub fn XCreateGC(_4: *mut Display, _3: c_ulong, _2: c_ulong, _1: *mut XGCValues) -> GC;
    pub fn XSetWMNormalHints(_3: *mut Display, _2: c_ulong, _1: *mut XSizeHints);
    pub fn XChangeProperty(
        _8: *mut Display,
        _7: c_ulong,
        _6: c_ulong,
        _5: c_ulong,
        _4: c_int,
        _3: c_int,
        _2: *const c_uchar,
        _1: c_int,
    ) -> c_int;
    pub fn XClearWindow(_2: *mut Display, _1: c_ulong) -> c_int;
    pub fn XMapRaised(_2: *mut Display, _1: c_ulong) -> c_int;
    pub fn XSetWMProtocols(_4: *mut Display, _3: c_ulong, _2: *mut c_ulong, _1: c_int) -> c_int;
    pub fn XFlush(_1: *mut Display) -> c_int;
    pub fn XDestroyWindow(_2: *mut Display, _1: c_ulong) -> c_int;
    pub fn XCreateImage(
        _10: *mut Display,
        _9: *mut Visual,
        _8: c_uint,
        _7: c_int,
        _6: c_int,
        _5: *mut c_char,
        _4: c_uint,
        _3: c_uint,
        _2: c_int,
        _1: c_int,
    ) -> *mut XImage;
    pub fn XDestroyImage(_1: *mut XImage) -> c_int;
    pub fn XStoreName(_3: *mut Display, _2: c_ulong, _1: *const c_char) -> c_int;
    pub fn XDefineCursor(_3: *mut Display, _2: c_ulong, _1: c_ulong) -> c_int;
    pub fn XCreateBitmapFromData(
        _5: *mut Display,
        _4: c_ulong,
        _3: *const c_char,
        _2: c_uint,
        _1: c_uint,
    ) -> c_ulong;
    pub fn XCreatePixmapCursor(
        _7: *mut Display,
        _6: c_ulong,
        _5: c_ulong,
        _4: *mut XColor,
        _3: *mut XColor,
        _2: c_uint,
        _1: c_uint,
    ) -> c_ulong;
    pub fn XMoveWindow(_4: *mut Display, _3: c_ulong, _2: c_int, _1: c_int) -> c_int;
    pub fn XGetWindowAttributes(_3: *mut Display, _2: c_ulong, _1: *mut XWindowAttributes)
        -> c_int;
    pub fn XTranslateCoordinates(
        _8: *mut Display,
        _7: c_ulong,
        _6: c_ulong,
        _5: c_int,
        _4: c_int,
        _3: *mut c_int,
        _2: *mut c_int,
        _1: *mut c_ulong,
    ) -> c_int;
    pub fn XPutImage(
        _10: *mut Display,
        _9: c_ulong,
        _8: GC,
        _7: *mut XImage,
        _6: c_int,
        _5: c_int,
        _4: c_int,
        _3: c_int,
        _2: c_uint,
        _1: c_uint,
    ) -> c_int;
    pub fn XQueryPointer(
        _9: *mut Display,
        _8: c_ulong,
        _7: *mut c_ulong,
        _6: *mut c_ulong,
        _5: *mut c_int,
        _4: *mut c_int,
        _3: *mut c_int,
        _2: *mut c_int,
        _1: *mut c_uint,
    ) -> c_int;
    pub fn XNextEvent(_2: *mut Display, _1: *mut XEvent) -> c_int;
    pub fn XPending(_1: *mut Display) -> c_int;
    pub fn XFilterEvent(_2: *mut XEvent, _1: c_ulong) -> c_int;
    pub fn XkbKeycodeToKeysym(_4: *mut Display, _3: c_uchar, _2: c_int, _1: c_int) -> c_ulong;
    pub fn XLookupKeysym(_2: *mut XKeyEvent, _1: c_int) -> c_ulong;
    pub fn Xutf8LookupString(
        _6: XIC,
        _5: *mut XKeyEvent,
        _4: *mut c_char,
        _3: c_int,
        _2: *mut c_ulong,
        _1: *mut c_int,
    ) -> c_int;
    pub fn XDestroyIC(_1: XIC);
    pub fn XCloseIM(_1: XIM) -> c_int;

}
