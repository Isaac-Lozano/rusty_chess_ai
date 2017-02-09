#![allow(dead_code)]

pub enum Color
{
    Black       = 0,
    Red         = 1,
    Green       = 2,
    Yellow      = 3,
    Blue        = 4,
    Magenta     = 5,
    Cyan        = 6,
    White       = 7,
}

const FOREGROUND_COLOR_BASE: u32 = 30;
const BACKGROUND_COLOR_BASE: u32 = 40;

pub fn get_foreground_color_code(color: Color) -> u32
{
    FOREGROUND_COLOR_BASE + (color as u32)
}

pub fn get_background_color_code(color: Color) -> u32
{
    BACKGROUND_COLOR_BASE + (color as u32)
}

pub fn gen_color_escape_code(fgcolor: u32, is_white_space: bool) -> String
{
    let bgcolor = get_background_color_code(if is_white_space { Color::Blue } else { Color::Black });

    format!("\x1b[1;{};{}m", fgcolor, bgcolor)
}

#[test]
fn test_get_foreground_color_code()
{
    assert_eq!(30, get_foreground_color_code(Color::Black));
    assert_eq!(33, get_foreground_color_code(Color::Yellow));
    assert_eq!(37, get_foreground_color_code(Color::White));
}

#[test]
fn test_get_background_color_code()
{
    assert_eq!(40, get_background_color_code(Color::Black));
    assert_eq!(43, get_background_color_code(Color::Yellow));
    assert_eq!(47, get_background_color_code(Color::White));
}

#[test]
fn test_gen_color_escape_code()
{
    assert_eq!("\x1b[1;31;40m", gen_color_escape_code(get_foreground_color_code(Color::Red), false));
    assert_eq!("\x1b[1;32;44m", gen_color_escape_code(get_foreground_color_code(Color::Green), true));
}


