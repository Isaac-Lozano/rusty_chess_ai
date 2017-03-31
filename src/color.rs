#![allow(dead_code)]

pub enum Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

pub fn get_color_escape_code(foreground_color: Color, background_color: Color) -> String {
    let foreground_color_code = get_foreground_color_code(foreground_color);
    let background_color_code = get_background_color_code(background_color);

    format!("\x1b[1;{};{}m",
            foreground_color_code,
            background_color_code)
}

pub fn get_color_reset_code() -> String {
    String::from("\x1b[0m")
}

fn get_foreground_color_code(color: Color) -> u32 {
    const FOREGROUND_COLOR_BASE: u32 = 30;
    FOREGROUND_COLOR_BASE + (color as u32)
}

fn get_background_color_code(color: Color) -> u32 {
    const BACKGROUND_COLOR_BASE: u32 = 40;
    BACKGROUND_COLOR_BASE + (color as u32)
}

#[test]
fn test_get_color_escape_code() {
    assert_eq!("\x1b[1;31;40m",
               get_color_escape_code(Color::Red, Color::Black));
    assert_eq!("\x1b[1;32;44m",
               get_color_escape_code(Color::Green, Color::Blue));
}

#[test]
fn test_get_color_reset_code() {
    assert_eq!("\x1b[0m", get_color_reset_code());
}

#[test]
fn test_get_foreground_color_code() {
    assert_eq!(30, get_foreground_color_code(Color::Black));
    assert_eq!(33, get_foreground_color_code(Color::Yellow));
    assert_eq!(37, get_foreground_color_code(Color::White));
}

#[test]
fn test_get_background_color_code() {
    assert_eq!(40, get_background_color_code(Color::Black));
    assert_eq!(43, get_background_color_code(Color::Yellow));
    assert_eq!(47, get_background_color_code(Color::White));
}
