use auto_array::auto_array;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub type KeyCode = u8;

macro_rules! keys {
    ($($name:tt => $value: tt)*) => {
        $(
            #[allow(non_upper_case_globals)]
            pub const $name: KeyCode = $value;
        )*

        pub static KEYBD_MAPPING: Lazy<HashMap<&str, KeyCode>> = Lazy::new(|| {
            let mut m = HashMap::new();
            $(
                m.insert(stringify!($name), $value);
            )*

            m
        });
    };
}

keys! {
    Backspace => 0x08
    Super => 0x5B
    Tab => 0x09
    Clear => 0x0c
    Enter => 0x0d
    Return => 0x0d
    Shift => 0x10
    Ctrl => 0x11
    Alt => 0x12
    Pause => 0x13
    Capslock => 0x14
    Kana => 0x15
    Hanguel => 0x15
    Hangul => 0x15
    Junja => 0x17
    Final => 0x18
    Hanja => 0x19
    Esc => 0x1b
    Escape => 0x1b
    Convert => 0x1c
    Noncovert => 0x1d
    Accept => 0x1e
    Modechange => 0x1f
    Space => 0x20
    PgUp => 0x21
    PgDn => 0x22
    PageUp => 0x21
    PageDown => 0x22
    End => 0x23
    Home => 0x24
    Left => 0x25
    Up => 0x26
    Right => 0x27
    Down => 0x28
    Select => 0x29
    Print => 0x2a
    Execute => 0x2b
    PrtSc => 0x2c
    PrintScreen => 0x2c
    Insert => 0x2d
    Del => 0x2e
    Delete => 0x2e
    Help => 0x2f
    Win => 0x5b
    WinLeft => 0x5b
    WinRight => 0x5c
    Apps => 0x5d
    Sleep => 0x5f
    Num0 => 0x60
    Num1 => 0x61
    Num2 => 0x62
    Num3 => 0x63
    Num4 => 0x64
    Num5 => 0x65
    Num6 => 0x66
    Num7 => 0x67
    Num8 => 0x68
    Num9 => 0x69
    Multiply => 0x6a
    Add => 0x6b
    Separator  => 0x6c
    Subtract => 0x6d
    Decimal => 0x6e
    Divide => 0x6f
    F1 => 0x70
    F2 => 0x71
    F3 => 0x72
    F4 => 0x73
    F5 => 0x74
    F6 => 0x75
    F7 => 0x76
    F8 => 0x77
    F9 => 0x78
    F10 => 0x79
    F11 => 0x7a
    F12 => 0x7b
    F13 => 0x7c
    F14 => 0x7d
    F15 => 0x7e
    F16 => 0x7f
    F17=> 0x80
    F18=> 0x81
    F19=> 0x82
    F20=> 0x83
    F21=> 0x84
    F22=> 0x85
    F23=>0x86
    F24=> 0x87
    Numlock => 0x90
    ScrollLock =>0x91
    ShiftLeft =>0xa0
    ShiftRight => 0xa1
    CtrlLeft => 0xa2
    CtrlRight => 0xa3
    AltLeft => 0xa4
    AltRight => 0xa5
    BrowserBack=>0xa6
    BrowserForward=> 0xa7
    BrowserRefresh=>0xa8
    BrowserStop=> 0xa9
    BrowserSearch=> 0xaa
    BrowserFavorites=> 0xab
    BrowserHome=> 0xac
    VolumeMute => 0xad
    VolumeDown => 0xae
    VolumeUp => 0xaf
    Stop => 0xb2
    PlayPause => 0xb3
    Launchmail=> 0xb4
    Launchmediaselect=> 0xb5
    Launchapp1=> 0xb6
    Launchapp2=> 0xb7
}

auto_array! {
    const KEY_NAMES: &str = [
        "\t",
        "\n",
        "\r",
        " ",
        "!",
        r#"""#,
        "#",
        "$",
        "%",
        "&",
        r#"""#,
        "(",
        ")",
        "*",
        "+",
        ",",
        "-",
        ".",
        "/",
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        ":",
        ";",
        "<",
        "=",
        ">",
        "?",
        "@",
        "[",
        "\\",
        "]",
        "^",
        "_",
        "`",
        "a",
        "b",
        "c",
        "d",
        "e",
        "f",
        "g",
        "h",
        "i",
        "j",
        "k",
        "l",
        "m",
        "n",
        "o",
        "p",
        "q",
        "r",
        "s",
        "t",
        "u",
        "v",
        "w",
        "x",
        "y",
        "z",
        "{",
        "|",
        "}",
        "~",
        "accept",
        "add",
        "alt",
        "altleft",
        "altright",
        "apps",
        "backspace",
        "browserback",
        "browserfavorites",
        "browserforward",
        "browserhome",
        "browserrefresh",
        "browsersearch",
        "browserstop",
        "capslock",
        "clear",
        "convert",
        "ctrl",
        "ctrlleft",
        "ctrlright",
        "decimal",
        "del",
        "delete",
        "divide",
        "down",
        "end",
        "enter",
        "esc",
        "escape",
        "execute",
        "f1",
        "f10",
        "f11",
        "f12",
        "f13",
        "f14",
        "f15",
        "f16",
        "f17",
        "f18",
        "f19",
        "f2",
        "f20",
        "f21",
        "f22",
        "f23",
        "f24",
        "f3",
        "f4",
        "f5",
        "f6",
        "f7",
        "f8",
        "f9",
        "final",
        "fn",
        "hanguel",
        "hangul",
        "hanja",
        "help",
        "home",
        "insert",
        "junja",
        "kana",
        "kanji",
        "launchapp1",
        "launchapp2",
        "launchmail",
        "launchmediaselect",
        "left",
        "modechange",
        "multiply",
        "nexttrack",
        "nonconvert",
        "num0",
        "num1",
        "num2",
        "num3",
        "num4",
        "num5",
        "num6",
        "num7",
        "num8",
        "num9",
        "numlock",
        "pagedown",
        "pageup",
        "pause",
        "pgdn",
        "pgup",
        "playpause",
        "prevtrack",
        "print",
        "printscreen",
        "prntscrn",
        "prtsc",
        "prtscr",
        "return",
        "right",
        "scrolllock",
        "select",
        "separator",
        "shift",
        "shiftleft",
        "shiftright",
        "sleep",
        "space",
        "stop",
        "subtract",
        "tab",
        "up",
        "volumedown",
        "volumemute",
        "volumeup",
        "win",
        "winleft",
        "winright",
        "yen",
        "command",
        "option",
        "optionleft",
        "optionright",
    ];
}

// pub static KEYBD_MAPPING: Lazy<HashMap<&str, KeyCode>> = Lazy::new(|| {
//     let mut m = HashMap::new();
//     m.insert("backspace", 0x08);
//     m
// });
