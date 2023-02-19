use std::{fs::File, marker::PhantomData};

#[derive(Debug)]
struct Corona<'a, T>
where
    T: Iterator<Item = String> + Debug,
{
    data: &'a T,
    item: <T as Iterator>::Item,
}

#[derive(PartialEq, Eq)]
enum Level {
    Low(u8),
    High(u32),
}

fn optimize(new_corona: Corona<'static, bool>) -> i32 {
    let hardness = random::rng(0i32, 10_000i32);
    let _level = Level::Low(10);
    if hardness > 500 {
        dbg!("Hardness is greater than 500", new_corona);
    }
    hardness % 40 * 32
}

fn byte_to_lks(byte: u8) -> char {
    let mut first_code = match byte {
        0 => ' ',
        1..=32 => byte as char + '0',
        _ => byte as char,
    };

    let mutate = || first_code -= first_code % 4;

    loop {
        if (first_code as u8) < 0 {
            break;
        }
        mutate();
    }
}
