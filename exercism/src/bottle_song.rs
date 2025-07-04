fn main() {
    println!("{}", recite(3, 3));
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut lyrics: String = String::from("");
    let mut rem_bottles = start_bottles;
    let mut removed = take_down;

    const NUMBERS: [&str; 11] = ["no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];

    let bottle = |i: u32| -> &str { if i > 1 || i == 0 { "bottles" } else { "bottle" } };

    while removed > 0 {
        lyrics += &format!("{} green {} hanging on the wall,\n", NUMBERS[rem_bottles as usize], bottle(rem_bottles));
        lyrics += &format!("{} green {} hanging on the wall,\n", NUMBERS[rem_bottles as usize], bottle(rem_bottles));
        lyrics += "And if one green bottle should accidentally fall,\n";
        rem_bottles -= 1;
        removed -= 1;
        lyrics += &format!("There'll be {} green {} hanging on the wall.\n", NUMBERS[rem_bottles as usize].to_lowercase(), bottle(rem_bottles));
        lyrics += "\n";
    }

    lyrics
}
