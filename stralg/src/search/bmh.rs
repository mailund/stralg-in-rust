use crate::utils::{CharacterTrait, Str, StrMapper, StrMappers};

struct BMHSearch<Char>
where
    Char: CharacterTrait,
{
    x: Str<Char>,
    p: Str<Char>,
    i: usize,
    bad_char_table: [usize; 256], // FIXME: The size must depend on the alphabet size.
}

fn build_bad_char_table<Char>(p: &Str<Char>) -> [usize; 256]
where
    Char: CharacterTrait,
{
    let mut bad_char_table = [p.len(); 256];
    for (i, c) in p.iter().enumerate() {
        bad_char_table[c.to_usize()] = i;
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
            while k > 0 && p[k] == x[*i + k] {
                k -= 1;
            }
            if k == 0 && p[k] == x[*i] {
                *i += m;
                return Some(*i - m);
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
