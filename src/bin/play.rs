use dungeoncrawl::*;
use std::io::{self, BufRead, Write};

// fn prompt() {
//     // pre_prompt();
//     print!("dungeoncrawl > ");
//     let _ = io::stdout().flush();
//     // let mut stdout = io::stdout();

//     // write!(stdout, "dungeoncrawl >");
//     let mut buf = String::new();
//     let stdin = io::stdin();
//     let mut handle = stdin.lock();
//     handle.read_line(&mut buf).unwrap();

//     println!("\n\nYou wrote: {}", buf);
// }

fn main() {
    // println!("before");
    // loop {
    //     println!();
    //     prompt();
    // }

    // println!("after");

    let mut gs = GameState::new();
    encounter(&mut gs);
}
