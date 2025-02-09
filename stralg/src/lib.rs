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

    pub struct BorderSearch<'a> {
        x: &'a str,
        p: &'a str,
        b: Vec<usize>,
        i: usize,
        j: usize,
    }

    impl<'a> BorderSearch<'a> {
        pub fn new(x: &'a str, p: &'a str) -> Self {
            let b = border_array(p);
            BorderSearch {
                x,
                p,
                b,
                i: 0,
                j: 0,
            }
        }
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

    impl<'a> Iterator for BorderSearch<'a> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            let BorderSearch { x, p, b, i, j } = self;
            let n = x.len();
            let m = p.len();
            while *i < n {
                while *j > 0 && &x[*i..*i + 1] != &p[*j..*j + 1] {
                    *j = b[*j - 1];
                }
                if &x[*i..*i + 1] == &p[*j..*j + 1] {
                    *j += 1;
                }
                *i += 1;
                if *j == m {
                    *j = b[*j - 1];
                    return Some(self.i - m);
                }
            }
            None
        }
    }

    pub fn border_search<'a>(x: &'a str, p: &'a str) -> BorderSearch<'a> {
        BorderSearch::new(x, p)
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
    }
}

pub use search::border_search;
pub use search::naive;
