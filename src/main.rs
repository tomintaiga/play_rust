mod level;
use level::Level;

/// Game entry point
fn main() {
    // File name for a game level
    let file_name = "level.txt";

    let level1 = Level::from_file(file_name);

    println!("Width: {} Height: {}", level1.width(), level1.height());
    println!("{level1}");
}
