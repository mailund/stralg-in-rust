mod search {
    pub fn naive<'a>(x: &'a str, p: &'a str) -> impl Iterator<Item = usize> + 'a {
        let n = x.len();
        let m = p.len();
        (0..=n - m).filter(move |i| {
            let mut j = 0;
            while j < m && &x[i + j..i + j + 1] == &p[j..j + 1] {
                j += 1;
            }
            j == m
        })
    }

    /// Computes the border array for the given pattern `p`.
    ///
    /// The border array is an array where the value at each index `i` is the length of the longest
    /// proper prefix of the substring `p[0..=i]` which is also a suffix of this substring.
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
    /// The strict border array `ba` is an array where the value at each index `i` is the length of the longest
    /// proper prefix of the substring `p[0..=i]` which is also a suffix of this substring, with the additional
    /// constraint that `p[ba[i]+1] != p[i+1]`.
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

    pub struct BorderSearch<'a> {
        /// The string we are searching in
        x: &'a str,
        /// The pattern we are searching for
        p: &'a str,
        /// The border array of the pattern
        border_array: Vec<usize>,
        /// The current index in the string
        x_index: usize,
        /// The current index in the pattern
        p_index: usize,
    }

    impl<'a> BorderSearch<'a> {
        pub fn new(x: &'a str, p: &'a str) -> Self {
            let b = strict_border_array(p);
            BorderSearch {
                x,
                p,
                border_array: b,
                x_index: 0,
                p_index: 0,
            }
        }
    }

    impl<'a> Iterator for BorderSearch<'a> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let BorderSearch {
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
                while *j > 0 && &x[*i..*i + 1] != &p[*j..*j + 1] {
                    *j = b[*j - 1];
                }

                // Move one step forward (if we can)
                if &x[*i..*i + 1] == &p[*j..*j + 1] {
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

    /// Returns an iterator over the starting indices of occurrences of the pattern `p` in the text `x`.
    ///
    /// # Examples
    ///
    /// ```
    /// use stralg::border_search;
    ///
    /// let text = "abracadabra";
    /// let pattern = "abr";
    /// let matches: Vec<usize> = border_search(text, pattern).collect();
    /// assert_eq!(matches, vec![0, 7]);
    /// ```
    pub fn border_search<'a>(x: &'a str, p: &'a str) -> BorderSearch<'a> {
        BorderSearch::new(x, p)
    }

    struct KMPSearch<'a> {
        /// The string we are searching in
        x: &'a str,
        /// The pattern we are searching for
        p: &'a str,
        /// The border array of the pattern
        border_array: Vec<usize>,
        /// The current index in the string
        x_index: usize,
        /// The current index in the pattern
        p_index: usize,
    }

    impl<'a> KMPSearch<'a> {
        fn new(x: &'a str, p: &'a str) -> KMPSearch<'a> {
            let b = strict_border_array(p);
            KMPSearch {
                x,
                p,
                border_array: b,
                x_index: 0,
                p_index: 0,
            }
        }
    }

    impl Iterator for KMPSearch<'_> {
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
                while *j > 0 && &x[*i..*i + 1] != &p[*j..*j + 1] {
                    *j = b[*j - 1];
                }

                // Move one step forward (if we can)
                if &x[*i..*i + 1] == &p[*j..*j + 1] {
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

    /// Returns an iterator over the starting indices of occurrences of the pattern `p` in the text `x`.
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_border_array() {
            let p = "abracadabra";
            let b = border_array(p);
            assert_eq!(b, vec![0, 0, 0, 1, 0, 1, 0, 1, 2, 3, 4]);
        }

        fn search_01<'a, F, I>(f: F)
        where
            F: Fn(&'a str, &'a str) -> I,
            I: Iterator<Item = usize> + 'a,
        {
            let x = "abracadabra";
            let p = "abr";
            let result: Vec<usize> = f(x, p).collect();
            assert_eq!(result, vec![0, 7]);
        }

        #[test]
        fn naive_01() {
            search_01(naive);
        }

        #[test]
        fn border_search_01() {
            search_01(border_search);
        }

        #[test]
        fn kmp_01() {
            search_01(kmp);
        }
    }
}

pub use search::border_array;
pub use search::border_search;
pub use search::kmp;
pub use search::naive;
pub use search::strict_border_array;
