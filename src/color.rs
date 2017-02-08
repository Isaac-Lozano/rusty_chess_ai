#![allow(dead_code)]

pub const FG_BLACK: u32     = 30;
pub const FG_RED: u32       = 31;
pub const FG_GREEN: u32     = 32;
pub const FG_YELLOW: u32    = 33;
pub const FG_BLUE: u32      = 34;
pub const FG_MAGENTA: u32   = 35;
pub const FG_CYAN: u32      = 36;
pub const FG_WHITE: u32     = 37;

pub const BG_BLACK: u32     = 40;
pub const BG_RED: u32       = 41;
pub const BG_GREEN: u32     = 42;
pub const BG_YELLOW: u32    = 43;
pub const BG_BLUE: u32      = 44;
pub const BG_MAGENTA: u32   = 45;
pub const BG_CYAN: u32      = 46;
pub const BG_WHITE: u32     = 47;

pub fn gen_color_escape_code(fgcolor: u32, is_white_space: bool) -> String
{
    let bgcolor = if is_white_space { BG_BLUE } else { BG_BLACK };

    format!("\x1b[1;{};{}m", fgcolor, bgcolor)
}


#[test]
fn test_gen_color_escape_code()
{
    assert_eq!("\x1b[1;31;40m", gen_color_escape_code(FG_RED, false));

    assert_eq!("\x1b[1;32;44m", gen_color_escape_code(FG_GREEN, true));
}
