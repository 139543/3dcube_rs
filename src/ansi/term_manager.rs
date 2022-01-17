use crate::ansi::color::Character;
use crate::ansi::ColorManager;
use crate::ansi::OffsetPosManager;

pub struct TermManager {
    buffer1: Vec<Vec<Character>>,
    buffer2: Vec<Vec<Character>>,
    pos: OffsetPosManager,
    color: ColorManager
}