use std::collections::HashSet;

fn _main() {
    let inputs = &["dlroW", "world", "zombies", "pants"];
    for a in anagrams_for("world", inputs) {
        println!("{}", a)
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let char_vec: Vec<char> = word.chars().collect();
    let mut pos = 0;
    let mut ret: Vec<String> = Vec::new();
    permutate(word, &char_vec, &mut pos, &mut ret);

    let mut hash: HashSet<&'a str> = HashSet::new();

    for a in possible_anagrams {
        if ret.contains(&a.to_string().to_lowercase()) {
            hash.insert(a);
        }
    }

    hash
}

fn permutate(word: &str, vec: &Vec<char>, pos: &mut usize, ret: &mut Vec<String>) {
    // println!("hello {} {}", *pos, vec.iter().count());
    for n in *pos..vec.len() {
        // println!("hello {} {}", *pos, vec.iter().count());
        let new_vec = swap(vec, *pos, n);
        // println!("{}: {:?}", pos, vec);

        *pos += 1;

        permutate(word, &new_vec, pos, ret);
        if *pos == new_vec.len() - 1 {
            // println!("{:?}", new_vec);
            let str: String = new_vec.iter().collect();
            if word.to_lowercase() != str.to_lowercase() {
                ret.push(str.to_lowercase());
            }
            //ret.append(new_vec.iter().collect())
        }

        *pos -= 1;
    }
}

fn swap(vec: &Vec<char>, pos1: usize, pos2: usize) -> Vec<char> {
    let mut new_vec = vec.to_owned();
    let temp = new_vec[pos1];
    new_vec[pos1] = vec[pos2];
    new_vec[pos2] = temp;

    new_vec
}