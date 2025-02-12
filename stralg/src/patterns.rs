use crate::utils::{CharacterTrait, Str};

/// Computes the border array for the given pattern `p`.
///
/// The border array is an array where the value at each index `i` is the length
/// of the longest proper prefix of the substring `p[0..=i]` which is also a
/// suffix of this substring.
///
/// The algorithm runs in O(m) time, where `m` is the length of the pattern.
///
/// # Arguments
///
/// * `p` - The pattern for which to compute the border array.
///
/// # Returns
///
/// A vector containing the border array of the pattern `p`.
///
/// # Examples
///
/// ```
/// use std::rc::Rc;
/// use stralg::{border_array, utils::{Alphabet, Str}};
///
/// let alphabet = Rc::new(Alphabet::from_str("abracadabra"));
/// let pattern: Str<u8> = Str::from_str("abracadabra", &alphabet).unwrap();
/// let borders = border_array(&pattern);
/// assert_eq!(borders, vec![0, 0, 0, 1, 0, 1, 0, 1, 2, 3, 4]);
/// ```
///
/// ```
/// use std::rc::Rc;
/// use stralg::{border_array, utils::{Alphabet, Str}};
///
/// let alphabet = Rc::new(Alphabet::from_str("a"));
/// let pattern: Str<u8> = Str::from_str("aaaa", &alphabet).unwrap();
/// let borders = border_array(&pattern);
/// assert_eq!(borders, vec![0, 1, 2, 3]);
/// ```
pub fn border_array<Char>(p: &Str<Char>) -> Vec<usize>
where
    Char: CharacterTrait,
{
    let m = p.len();
    let mut ba = vec![0; m];
    let mut j = 0;
    for i in 1..m {
        while j > 0 && &p[i] != &p[j] {
            j = ba[j - 1];
        }
        if &p[i] == &p[j] {
            j += 1;
        }
        ba[i] = j;
    }
    ba
}

/// Computes the strict border array for the given pattern `p`.
///
/// The strict border array `ba` is an array where the value at each index `i`
/// is the length of the longest proper prefix of the substring `p[0..=i]`
/// which is also a suffix of this substring, with the additional constraint
/// that `p[ba[i]+1] != p[i+1]`.
///
/// The algorithm runs in O(m) time, where `m` is the length of the pattern.
///
/// # Arguments
///
/// * `p` - The pattern for which to compute the strict border array.
///
/// # Returns
///
/// A vector containing the strict border array of the pattern `p`.
///
/// # Examples
///
/// ```
/// use std::rc::Rc;
/// use stralg::{strict_border_array, utils::{Alphabet, Str}};
///
/// let alphabet = Rc::new(Alphabet::from_str("abracadabra"));
/// let pattern: Str<u8> = Str::from_str("abracadabra", &alphabet).unwrap();
///
/// let borders = strict_border_array(&pattern);
/// assert_eq!(borders, vec![0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 4]);
/// ```
///
/// ```
/// use std::rc::Rc;
/// use stralg::{strict_border_array, utils::{Alphabet, Str}};
///
/// let alphabet = Rc::new(Alphabet::from_str("a"));
/// let pattern: Str<u8> = Str::from_str("aaaa", &alphabet).unwrap();
///
/// let borders = strict_border_array(&pattern);
/// assert_eq!(borders, vec![0, 0, 0, 3]);
/// ```
///
/// ```
/// use std::rc::Rc;
/// use stralg::{strict_border_array, utils::{Alphabet, Str}};
///
/// let alphabet = Rc::new(Alphabet::from_str("abcd"));
/// let pattern: Str<u8> = Str::from_str("abcd", &alphabet).unwrap();
///
/// let borders = strict_border_array(&pattern);
/// assert_eq!(borders, vec![0, 0, 0, 0]);
/// ```
pub fn strict_border_array<Char>(p: &Str<Char>) -> Vec<usize>
where
    Char: CharacterTrait,
{
    let mut ba = border_array(p);
    for j in 1..(ba.len() - 1) {
        let b = ba[j];
        if b > 0 && p[b] == p[j + 1] {
            ba[j] = ba[b - 1];
        }
    }
    ba
}

#[cfg(test)]
mod tests {
    use crate::Alphabet;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn test_border_array() {
        let p = "abracadabra";
        let alpha = Rc::new(Alphabet::from_str(p));
        let p: Str<u8> = Str::from_str(p, &alpha).unwrap();
        let b = border_array(&p);
        assert_eq!(b, vec![0, 0, 0, 1, 0, 1, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_strict_border_array() {
        let p = "abracadabra";
        let alpha = Rc::new(Alphabet::from_str(p));
        let p: Str<u8> = Str::from_str(p, &alpha).unwrap();
        let b = strict_border_array(&p);
        assert_eq!(b, vec![0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 4]);
    }
}
