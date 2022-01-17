use cube3d_rs::ansi::OffsetPosManager;
fn main() {
    println!("Hello, world!");
    let mut pos = OffsetPosManager::new();
    pos.gotopos(3,7);
    pos.print("Hello, world!");
}
