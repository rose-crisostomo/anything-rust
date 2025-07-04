fn main() {
    println!("{}", is_armstrong_number(154))
}

pub fn is_armstrong_number(num: u32) -> bool {
    let mut a = num;
    let b = 10;
    let mut m = 0;
    let mut digits: Vec<u32> = Vec::new();

    loop {
        if a < b {
            digits.push(a);
            break;
        }

        (a, m) = (a / b, a % b);
        digits.push(m);

        if m == 0 {
            break;
        }
    }

    let mut sum = 0;
    let count: u32 = digits.len() as u32;
    for digit in digits {
        sum += digit.pow(count);
    }

    sum == num
}
