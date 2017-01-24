#[macro_use]
extern crate enum_primitive;

enum_from_primitive! {
/// Default values for hardware keyboard keys.
///
/// After USB HID Usage Tables document at http://www.usb.org/developers/hidpage/Hut1_12v2.pdf
///
/// You can use the `FromPrimitive` trait to convert from an USB HID integer into a `Scancode` enum
/// value.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Scancode {
    A = 4,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    N,
    M,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num1 = 30,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Enter = 40,
    Escape,
    Backspace,
    Tab,
    Space,
    Minus,
    Equals,
    LeftBracket,
    RightBracket,
    Backslash,
    NonUsHash = 50,
    Semicolon,
    Apostrophe,
    Grave,
    Comma,
    Period,
    Slash,
    CapsLock,
    F1 = 58,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrintScreen = 70,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    Delete,
    End,
    PageDown,
    Right,
    Left,
    Down,
    Up,
    NumLock = 83,
    PadDivide,
    PadMultiply,
    PadMinus,
    PadPlus,
    PadEnter,
    Pad1 = 89,
    Pad2,
    Pad3,
    Pad4,
    Pad5,
    Pad6,
    Pad7,
    Pad8,
    Pad9,
    Pad0,
    PadDecimal,
    NonUsBackslash = 100,
    PadEquals = 103,
    Menu = 118,
    Mute = 127,
    VolumeUp,
    VolumeDown,
    SysReq = 154,
    LeftControl = 224,
    LeftShift,
    LeftAlt,
    LeftGui,
    RightControl,
    RightShift,
    RightAlt,
    RightGui,
}
}

#[cfg(target_os = "macos")]
mod scancode_macos;
#[cfg(target_os = "linux")]
mod scancode_linux;
#[cfg(target_os = "windows")]
mod scancode_windows;

mod scancode {
    #[cfg(target_os = "macos")]
    pub use scancode_macos::MAP;
    #[cfg(target_os = "linux")]
    pub use scancode_linux::MAP;
    #[cfg(target_os = "windows")]
    pub use scancode_windows::MAP;
}

impl Scancode {
    /// Try to convert a hardware scancode from the current platform to a Scancode enum value.
    pub fn new(hardware_scancode: u8) -> Option<Scancode> {
        if (hardware_scancode as usize) < scancode::MAP.len() {
            scancode::MAP[hardware_scancode as usize]
        } else {
            None
        }
    }
}
