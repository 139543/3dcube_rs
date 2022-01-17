pub fn ansicmd(arg: &str) {
    print!("\x1B[{}",arg);
}