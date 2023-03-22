use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //println!("Searhing for {}", config.query);
    //println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    //    fs::write(        "poem.txt",
    //       "I'm nobody! Who are you?
    //Are you nobody, too?
    //Then there's a pair of us — don't tell!
    //They'd banish us, you know.
    //How dreary to be somebody!
    //How public, like a frog
    //To tell your name the livelong day
    //To an admiring bog!",
    //  )
    //  .expect("something wrong");
}
