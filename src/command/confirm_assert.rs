//交互式中断确认
use std::io;

// asert yes or no
pub fn assert_yes(message:String) -> bool{
    let show_msg = String::from(message)+"[Y/n]";
    println!("{} ", show_msg);
    let mut read_line = String::new();
    io::stdin().read_line(&mut read_line)
                .expect("Failed to read line.");
    read_line = String::from(read_line.trim());
    match read_line == "Y" || read_line == "y" {
        true => {
            return true;
        }
        _ => {
            return false;
        }
    }
}
