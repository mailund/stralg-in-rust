use crate::strict_border_array;

struct KMPSearch {
    /// The string we are searching in
    x: Vec<char>,
    /// The pattern we are searching for
    p: Vec<char>,
    /// The border array of the pattern
    border_array: Vec<usize>,
    /// The current index in the string
    x_index: usize,
    /// The current index in the pattern
    p_index: usize,
}

impl KMPSearch {
    fn new(x: &str, p: &str) -> KMPSearch {
        let b = strict_border_array(p);
        let x: Vec<char> = x.chars().collect();
        let p: Vec<char> = p.chars().collect();
        KMPSearch {
            x: x,
            p: p,
            border_array: b,
            x_index: 0,
            p_index: 0,
        }
    }
}

impl Iterator for KMPSearch {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let KMPSearch {
            x,
            p,
            border_array: b,
            x_index: i,
            p_index: j,
            ..
        } = self;

        let n = x.len();
        let m = p.len();

        while *i < n {
            // Shift pattern until it matches the border
            while *j > 0 && &x[*i] != &p[*j] {
                *j = b[*j - 1];
            }

            // Move one step forward (if we can)
            if &x[*i] == &p[*j] {
                *j += 1;
            }

            // Move to the next position (saving it for the next call to next)
            *i += 1;

            // Return if a match was found
            if *j == m {
                *j = b[*j - 1];
                return Some(*i - m);
            }
        }
        None
    }
}

/// Returns an iterator over the starting indices of occurrences of the pattern
/// `p` in the text `x` using the Knuth-Morris-Pratt (KMP) string matching algorithm.
///
/// The KMP algorithm uses the border array to efficiently find occurrences of
/// the pattern `p` in the text `x`. It shifts the pattern according to the
/// border array to avoid unnecessary comparisons.
///
/// The algorithm runs in O(n + m) time, where `n` is the length of the text, and
/// `m` is the length of the pattern.
///
/// # Arguments
///
/// * `x` - The text in which to search for the pattern.
/// * `p` - The pattern to search for.
///
/// # Returns
///
/// An iterator over the starting indices of occurrences of the pattern `p` in
/// the text `x`.
///
/// # Examples
///
/// ```
/// use stralg::kmp;
///
/// let text = "abracadabra";
/// let pattern = "abr";
/// let matches: Vec<usize> = kmp(text, pattern).collect();
/// assert_eq!(matches, vec![0, 7]);
/// ```
pub fn kmp<'a>(x: &'a str, p: &'a str) -> impl Iterator<Item = usize> + 'a {
    KMPSearch::new(x, p)
}
