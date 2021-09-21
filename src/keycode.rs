pub mod key_code {
    use rdev::Key;
    /// Option
    const ALT: i32 = 56;
    /// Option_Right
    const ALT_GR: i32 = 3640;
    const BACKSPACE: i32 = 14;
    const CAPS_LOCK: i32 = 58;
    /// Control Right does not exist on Mac
    const CONTROL_LEFT: i32 = 29;
    const DOWN_ARROW: i32 = 57424;
    const ESCAPE: i32 = 1;
    const F1: i32 = 59;
    const F10: i32 = 68;
    const F11: i32 = 87;
    const F12: i32 = 88;
    const F2: i32 = 60;
    const F3: i32 = 61;
    const F4: i32 = 62;
    const F5: i32 = 63;
    const F6: i32 = 64;
    const F7: i32 = 65;
    const F8: i32 = 66;
    const F9: i32 = 67;
    const FUNCTION: i32 = 3666;
    const LEFT_ARROW: i32 = 57419;
    const META_LEFT: i32 = 3675;
    const META_RIGHT: i32 = 3676;
    const RETURN: i32 = 28;
    const RIGHT_ARROW: i32 = 57421;
    const SHIFT_LEFT: i32 = 42;
    const SHIFT_RIGHT: i32 = 54;
    const SPACE: i32 = 57;
    const TAB: i32 = 15;
    const UP_ARROW: i32 = 57416;
    const BACK_QUOTE: i32 = 41;

    const NUM1: i32 = 2;
    const NUM2: i32 = 3;
    const NUM3: i32 = 4;
    const NUM4: i32 = 5;
    const NUM5: i32 = 6;
    const NUM6: i32 = 7;
    const NUM7: i32 = 8;
    const NUM8: i32 = 9;
    const NUM9: i32 = 10;
    const NUM0: i32 = 11;
    const MINUS: i32 = 12;
    const EQUAL: i32 = 13;

    const KEY_Q: i32 = 16;
    const KEY_W: i32 = 17;
    const KEY_E: i32 = 18;
    const KEY_R: i32 = 19;
    const KEY_T: i32 = 20;
    const KEY_Y: i32 = 21;
    const KEY_U: i32 = 22;
    const KEY_I: i32 = 23;
    const KEY_O: i32 = 24;
    const KEY_P: i32 = 25;

    const LEFT_BRACKET: i32 = 26;
    const RIGHT_BRACKET: i32 = 27;

    const KEY_A: i32 = 30;
    const KEY_S: i32 = 31;
    const KEY_D: i32 = 32;
    const KEY_F: i32 = 33;
    const KEY_G: i32 = 34;
    const KEY_H: i32 = 35;
    const KEY_J: i32 = 36;
    const KEY_K: i32 = 37;
    const KEY_L: i32 = 38;
    const SEMI_COLON: i32 = 39;
    const QUOTE: i32 = 40;
    const BACK_SLASH: i32 = 43;

    const KEY_Z: i32 = 44;
    const KEY_X: i32 = 45;
    const KEY_C: i32 = 46;
    const KEY_V: i32 = 47;
    const KEY_B: i32 = 48;
    const KEY_N: i32 = 49;
    const KEY_M: i32 = 50;
    const COMMA: i32 = 51;
    const DOT: i32 = 52;
    const SLASH: i32 = 53;

    
    pub fn code_from_key(key: Key) -> Option<i32> {
        match key {
            Key::Alt => Some(ALT),
            Key::AltGr => Some(ALT_GR),
            Key::Backspace => Some(BACKSPACE),
            Key::CapsLock => Some(CAPS_LOCK),
            Key::ControlLeft => Some(CONTROL_LEFT),
            Key::DownArrow => Some(DOWN_ARROW),
            Key::Escape => Some(ESCAPE),
            Key::F1 => Some(F1),
            Key::F10 => Some(F10),
            Key::F11 => Some(F11),
            Key::F12 => Some(F12),
            Key::F2 => Some(F2),
            Key::F3 => Some(F3),
            Key::F4 => Some(F4),
            Key::F5 => Some(F5),
            Key::F6 => Some(F6),
            Key::F7 => Some(F7),
            Key::F8 => Some(F8),
            Key::F9 => Some(F9),
            Key::LeftArrow => Some(LEFT_ARROW),
            Key::MetaLeft => Some(META_LEFT),
            Key::MetaRight => Some(META_RIGHT),
            Key::Return => Some(RETURN),
            Key::RightArrow => Some(RIGHT_ARROW),
            Key::ShiftLeft => Some(SHIFT_LEFT),
            Key::ShiftRight => Some(SHIFT_RIGHT),
            Key::Space => Some(SPACE),
            Key::Tab => Some(TAB),
            Key::UpArrow => Some(UP_ARROW),
            Key::BackQuote => Some(BACK_QUOTE),
            Key::Num1 => Some(NUM1),
            Key::Num2 => Some(NUM2),
            Key::Num3 => Some(NUM3),
            Key::Num4 => Some(NUM4),
            Key::Num5 => Some(NUM5),
            Key::Num6 => Some(NUM6),
            Key::Num7 => Some(NUM7),
            Key::Num8 => Some(NUM8),
            Key::Num9 => Some(NUM9),
            Key::Num0 => Some(NUM0),
            Key::Minus => Some(MINUS),
            Key::Equal => Some(EQUAL),
            Key::KeyQ => Some(KEY_Q),
            Key::KeyW => Some(KEY_W),
            Key::KeyE => Some(KEY_E),
            Key::KeyR => Some(KEY_R),
            Key::KeyT => Some(KEY_T),
            Key::KeyY => Some(KEY_Y),
            Key::KeyU => Some(KEY_U),
            Key::KeyI => Some(KEY_I),
            Key::KeyO => Some(KEY_O),
            Key::KeyP => Some(KEY_P),
            Key::LeftBracket => Some(LEFT_BRACKET),
            Key::RightBracket => Some(RIGHT_BRACKET),
            Key::KeyA => Some(KEY_A),
            Key::KeyS => Some(KEY_S),
            Key::KeyD => Some(KEY_D),
            Key::KeyF => Some(KEY_F),
            Key::KeyG => Some(KEY_G),
            Key::KeyH => Some(KEY_H),
            Key::KeyJ => Some(KEY_J),
            Key::KeyK => Some(KEY_K),
            Key::KeyL => Some(KEY_L),
            Key::SemiColon => Some(SEMI_COLON),
            Key::Quote => Some(QUOTE),
            Key::BackSlash => Some(BACK_SLASH),
            Key::KeyZ => Some(KEY_Z),
            Key::KeyX => Some(KEY_X),
            Key::KeyC => Some(KEY_C),
            Key::KeyV => Some(KEY_V),
            Key::KeyB => Some(KEY_B),
            Key::KeyN => Some(KEY_N),
            Key::KeyM => Some(KEY_M),
            Key::Comma => Some(COMMA),
            Key::Dot => Some(DOT),
            Key::Slash => Some(SLASH),
            Key::Function => Some(FUNCTION),
            _ => None
        }
    }
}