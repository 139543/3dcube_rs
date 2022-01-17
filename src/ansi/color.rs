#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
pub struct Color {
    main_code: String,
}

impl Color {
    pub fn fgcode(&self) -> String {
        "3".to_string() + &self.main_code
    }
    pub fn bgcode(&self) -> String {
        "4".to_string() + &self.main_code
    }
    pub fn new(code: String) -> Self {
        Color {
            main_code: code
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            main_code: "0".to_string()
        }
    }
}

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Default)]
#[derive(Clone)]
pub struct Character {
    fg_color: Color,
    bg_color: Color
}
