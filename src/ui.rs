
use std::io;
///

pub fn ask_for_file_name() -> String {
    println!("input filename for evaluation");
    let mut buffer = String::new();
    let buffer = loop {
        io::stdin().read_line(&mut buffer).expect("readline failed in 'ask_for_file_name'");
        if buffer.trim().contains(".") {
            println!("invalid input: only file name needed.");
            continue;
        }
        break buffer.trim().to_owned();
    };
    buffer
}







