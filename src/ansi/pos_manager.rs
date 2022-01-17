pub struct PosManager {
    x: usize,
    y: usize,
}

impl PosManager {
    pub fn gotopos(&mut self, desired_x: usize, desired_y: usize) {
        if self.x != desired_x || self.y != desired_y {
            //don't need arguments to go to top left corner
            if desired_x == 0 && desired_y == 0 {
                self.x = 0;
                self.y = 0;
                print!("\x1B[H");
            //only run this if row and column are different
            } else if desired_y != self.y {
                self.x = desired_x;
                self.y = desired_y;
                print!("\x1B[{};{}H", desired_y + 1, desired_x + 1);
            } else {
                if desired_x > self.x {
                    print!("\x1B[{}C",desired_x - self.x);
                } else {
                    print!("\x1B[{}D",self.x - desired_x);
                }
                self.x = desired_x;
            }
        }
    }
    pub fn new() -> PosManager {
        print!("\x1B[H");
        PosManager {x: 0, y: 0}
    }
    pub fn print(&mut self, text: &str) -> usize {
        print!("{}",text);
        let count = text.chars().count();
        self.x += count;
        count
    }
}

pub struct OffsetPosManager {
    pos: PosManager,
    x_offset: usize,
    y_offset: usize
}

impl OffsetPosManager {
    pub fn gotopos(&mut self, desired_x: usize, desired_y: usize) {
        self.pos.gotopos(self.x_offset + desired_x, self.y_offset + desired_y)
    }
    pub fn rawgotopos(&mut self, desired_x: usize, desired_y: usize) {
        self.pos.gotopos(desired_x, desired_y)
    }
    pub fn new() -> OffsetPosManager {
        OffsetPosManager {
            pos: PosManager::new(),
            x_offset: 0,
            y_offset: 0
        }
    }
    pub fn print(&mut self, text: &str) -> usize {
        self.pos.print(text)
    }
}