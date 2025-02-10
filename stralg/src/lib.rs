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

    fn border_array(p: &str) -> Vec<usize> {
        let m = p.len();
        let mut b = vec![0; m];
        let mut j = 0;
        for i in 1..m {
            while j > 0 && &p[i..i + 1] != &p[j..j + 1] {
                j = b[j - 1];
            }
            if &p[i..i + 1] == &p[j..j + 1] {
                j += 1;
            }
            b[i] = j;
        }
        b
    }

    /*
    fn filter_border_array(ba: Vec<usize>) -> Vec<usize> {
        for (j, b) in ba.iter().enumerate() {
            if j == ba.len() {
                break;
            }
            if *b > 0 {
                let mut k = j - 1;
                while k > 0 {
                    if ba[k] == *b {
                        ba[k] = 0;
                    }
                    k -= 1;
                }
            }
        }
    }*/

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
            let b = border_array(p);
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
            let b = border_array(p);
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

pub use search::border_search;
pub use search::kmp;
pub use search::naive;
