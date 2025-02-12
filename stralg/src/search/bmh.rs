use core::panic;

use crate::utils::{CharacterTrait, Str, StrMapper, StrMappers};

struct BMHSearch<Char>
where
    Char: CharacterTrait,
{
    x: Str<Char>,
    p: Str<Char>,
    i: usize,
    bad_char_table: Vec<usize>,
}

fn build_bad_char_table<Char>(p: &Str<Char>) -> Vec<usize>
where
    Char: CharacterTrait,
{
    let mut bad_char_table = vec![p.len(); p.alphabet.len() + 1];
    for i in 0..(p.len() - 1) {
        bad_char_table[p[i].to_usize()] = p.len() - i - 1;
    }
    bad_char_table
}

impl<Char> BMHSearch<Char>
where
    Char: CharacterTrait,
{
    fn new(x: Str<Char>, p: Str<Char>) -> BMHSearch<Char> {
        let bad_char_table = build_bad_char_table(&p);
        BMHSearch {
            x,
            p,
            i: 0,
            bad_char_table,
        }
    }
}

impl<Char: CharacterTrait> Iterator for BMHSearch<Char> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let BMHSearch {
            x,
            p,
            i,
            bad_char_table,
        } = self;
        let n = x.len();
        let m = p.len();
        while *i <= n - m {
            let mut k = m - 1;
            println!("0: i: {}, k: {}", i, k);
            while k > 0 && p[k] == x[*i + k] {
                println!("1: i: {}, k: {}", i, k);
                k -= 1;
            }
            println!("2: i: {}, k: {}", i, k);
            if k == 0 && p[k] == x[*i] {
                let jump = bad_char_table[x[*i].to_usize()];
                println!("jump: {}", jump);
                if jump == 0 {
                    panic!("jump == 0");
                }
                if jump == 0 {
                    *i += 1;
                } else {
                    *i += jump;
                }
                let hit = *i;
                *i += bad_char_table[x[*i + m - 1].to_usize()];
                println!("3: i: {}, k: {}", i, k);
                println!("hit: {}", hit);
                return Some(hit);
            }
            let jump = bad_char_table[x[*i].to_usize()];
            println!("jump2: {}", jump);
            if jump == 0 {
                panic!("jump == 0");
            }
            *i += bad_char_table[x[*i + m - 1].to_usize()];
        }
        None
    }
}

fn bmh_impl<Char>(x: &str, p: &str, mapper: StrMapper<Char>) -> Box<dyn Iterator<Item = usize>>
where
    Char: CharacterTrait,
{
    let x = mapper.map_str(x).unwrap(); // We built the string from x so this cannot fail...
    let p = match mapper.map_str(p) {
        Ok(p) => p,
        Err(_) => return Box::new(std::iter::empty()),
    };
    Box::new(BMHSearch::new(x, p))
}

pub fn bmh(x: &str, p: &str) -> Box<dyn Iterator<Item = usize>> {
    if x.is_empty() || p.is_empty() {
        return Box::new(std::iter::empty());
    }
    if x.len() < p.len() {
        return Box::new(std::iter::empty());
    }

    let mapper = StrMappers::new_from_str(x).unwrap(); // We unwrap because we don't expect alphabet larger than u16
    match mapper {
        StrMappers::U8Mapper(mapper) => bmh_impl(x, p, mapper),
        StrMappers::U16Mapper(mapper) => bmh_impl(x, p, mapper),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{utils::Str, Alphabet};
    use std::rc::Rc;

    #[test]
    fn test_bad_charater_table() {
        let p = "abracadabra"; // len = 11
        let alphabet = Rc::new(Alphabet::from_str(p)); // $abcdr
        let p: Str<u8> = Str::from_str(p, &alphabet).unwrap();
        // Jumps table (right-most index from the right, or full pattern if unknown)
        // $ -> 11, a -> 3, b -> 2, c -> 6, d -> 4, r -> 1
        let result = build_bad_char_table(&p);
        assert_eq!(result, vec![11, 3, 2, 6, 4, 1]);
    }
}
