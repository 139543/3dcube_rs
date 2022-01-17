pub struct Color {
    main_code: String,
}
impl Color {
    pub fn get_fgcode(&self) -> String {
        "3".to_string() + &self.main_code
    }
    pub fn get_bgcode(&self) -> String {
        "4".to_string() + &self.main_code
    }
}