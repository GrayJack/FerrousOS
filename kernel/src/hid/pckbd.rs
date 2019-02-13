//! # PS/2 Keyboard Driver
//!
//! Allows for simple keyboard input

use crate::hid::queue::Queue;

pub enum KeyCode {
    At, Excl, Slash, BackSlash, Dot, Comma, Tick,
    N0, N1, N2, N3, N4, N5, N6, N7, N8, N9,
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z
}

pub enum ScanCode {
    Press(KeyCode),
    Release(KeyCode)
}

pub struct PCKeyboard {
    command_queue: Queue<'static, u8>
}

impl PCKeyboard {
    pub fn new() -> PCKeyboard {
        unimplemented!()
    }
}
