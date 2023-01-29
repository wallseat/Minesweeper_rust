use crate::draw::draw;
use crate::game::State;

pub fn num_to_str_cord_name(num: usize) -> String {
    let mut v = Vec::<String>::new();
    let mut d: usize = num;
    loop {
        v.push(
            char::from_u32((('a' as usize) + (d % 26)) as u32)
                .unwrap()
                .to_string(),
        );
        d /= 26;
        if d == 0 {
            break;
        }
    }
    v.reverse();
    v.join("")
}

pub fn str_cord_name_to_num(str: String) -> Result<usize, ()> {
    let count = str.chars().count();
    let mut num = 0;

    for (idx, letter) in str.chars().enumerate().into_iter() {
        if !letter.is_ascii_alphabetic() {
            return Err(());
        }

        let mut p = (letter as u32 - 'a' as u32) as usize;
        let rank = count - idx - 1;

        if rank != 0 {
            p += 1
        }

        num += p * usize::pow(26, rank as u32)
    }

    return Ok(num);
}

pub fn clear() {
    print!("{esc}c", esc = 27 as char)
}

pub fn flip(state: &State) {
    clear();
    draw(&state);
}