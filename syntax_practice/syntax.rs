use std::fs;
use std::io::Read;
use std::io::Write;

fn main() -> std::io::Result<()> {
    fs::write("Shadows.txt", "I see.")?;

    let mut file_holder = fs::File::open("Shadows.txt")?;

    let mut file_string = String::new();

    file_holder.read_to_string(&mut file_string)?;

    file_string
        .split_whitespace()
        .for_each(|word| println!("{}", word.to_uppercase()));
    let mut str = String::from("Someone to die for.");

    let string = cut_till_space(&mut str);

    println!("str: {}", string);
    Ok(())
}

fn cut_till_space(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

