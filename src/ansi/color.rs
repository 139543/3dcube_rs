#[derive(Eq)]
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

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.main_code == other.main_code
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            main_code: "0".to_string()
        }
    }
}