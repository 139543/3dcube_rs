use super::Color;
pub struct ColorManager {
    fg: Color,
    bg: Color
}

impl ColorManager {
    pub fn set_fg_color(&mut self, color: Color) {
        if color != self.fg {
            self.fg = color;
            print!("\x1B[{}m",self.fg.fgcode());
        }
    }
    pub fn set_bg_color(&mut self, color: Color) {
        if color != self.bg {
            self.bg = color;
            print!("\x1B[{}m",self.bg.bgcode());
        }
    }
    pub fn setcolors(&mut self, color1: Color, color2: Color) {
        if color1 != self.fg && color2 != self.bg {
            self.fg = color1;
            self.bg = color2;
            print!("\x1B[{};{}m",self.fg.fgcode(),self.bg.bgcode());
        } else {
            self.set_fg_color(color1);
            self.set_bg_color(color2);
        }
    }
    pub fn new() -> Self {
        print!("\x1B[30;40m");
        ColorManager {
            fg: Color::default(),
            bg: Color::default()
        }
    }
}