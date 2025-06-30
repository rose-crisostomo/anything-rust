// study rust map
fn main() {
    let code: &str = "9999999999 9999999999 9999999999 9999999999";

    const ZERO_ASCII: u8 = b'0';
    const NINE_ASCII: u8 = b'9';
    const SPACE_ASCII: u8 = b' ';

    let mut skip = true;
    let mut sum: u32 = 0;

    for c in code.as_bytes().iter().rev() {
        match c {
            ZERO_ASCII ..= NINE_ASCII => {
                let digit: u32 = (c - ZERO_ASCII).into();
                if !skip {
                    let product: u32 = digit * 2;
                    sum += product - (if product > 9 { 9 } else { 0 });
                    skip = true;
                } else {
                    skip = false;
                    sum += digit;
                }
            },
            &SPACE_ASCII => continue,
            _ => {
                sum = 0;
                break;
            }
        };
    }

    println!("{}", sum % 10);
    if (sum > 0 && sum % 10 == 0) || (sum == 0 && code.matches('0').count() > 1) {
        println!("valid {sum}")
    } else {
        println!("invalid {sum}")
    }
}