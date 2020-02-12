use rltk::{Console, font, RGB, Rltk, SimpleConsole};

use crate::{TITLE, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct ContextBuilder<'a> {
    pub width: u32,
    pub height: u32,
    pub title: &'a str,
}

