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
/// use stralg::border_array;
///
/// let pattern = "abracadabra";
/// let borders = border_array(pattern);
/// assert_eq!(borders, vec![0, 0, 0, 1, 0, 1, 0, 1, 2, 3, 4]);
/// ```
///
/// ```
/// use stralg::border_array;
///
/// let pattern = "aaaa";
/// let borders = border_array(pattern);
/// assert_eq!(borders, vec![0, 1, 2, 3]);
/// ```
///
/// ```
/// use stralg::border_array;
///
/// let pattern = "abcd";
/// let borders = border_array(pattern);
/// assert_eq!(borders, vec![0, 0, 0, 0]);
/// ```
pub fn border_array(p: &str) -> Vec<usize> {
    let m = p.len();
    let mut ba = vec![0; m];
    let mut j = 0;
    for i in 1..m {
        while j > 0 && &p[i..i + 1] != &p[j..j + 1] {
            j = ba[j - 1];
        }
        if &p[i..i + 1] == &p[j..j + 1] {
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
/// use stralg::strict_border_array;
///
/// let pattern = "abracadabra";
/// let borders = strict_border_array(pattern);
/// assert_eq!(borders, vec![0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 4]);
/// ```
///
/// ```
/// use stralg::strict_border_array;
///
/// let pattern = "aaaa";
/// let borders = strict_border_array(pattern);
/// assert_eq!(borders, vec![0, 0, 0, 3]);
/// ```
///
/// ```
/// use stralg::strict_border_array;
///
/// let pattern = "abcd";
/// let borders = strict_border_array(pattern);
/// assert_eq!(borders, vec![0, 0, 0, 0]);
/// ```
pub fn strict_border_array(p: &str) -> Vec<usize> {
    let mut ba = border_array(p);
    for j in 1..(ba.len() - 1) {
        let b = ba[j];
        if b > 0 && p[b..b + 1] == p[j + 1..j + 2] {
            ba[j] = ba[b - 1];
        }
    }
    ba
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_array() {
        let p = "abracadabra";
        let b = border_array(p);
        assert_eq!(b, vec![0, 0, 0, 1, 0, 1, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_strict_border_array() {
        let p = "abracadabra";
        let b = strict_border_array(p);
        assert_eq!(b, vec![0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 4]);
    }
}
