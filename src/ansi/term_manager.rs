use crate::ansi::color::Character;
use crate::ansi::ColorManager;
use crate::ansi::OffsetPosManager;

pub struct TermManager {
    buffer1: Vec<Vec<Character>>,
    buffer2: Vec<Vec<Character>>,
    pos: OffsetPosManager,
    color: ColorManager
}

impl TermManager {
    pub fn new() -> Self {
        TermManager {
            buffer1: Vec::new(),
            buffer2: Vec::new(),
            pos: OffsetPosManager::new(),
            color: ColorManager::new()
        }
    }
}