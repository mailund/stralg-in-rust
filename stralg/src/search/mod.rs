pub mod kmp;
pub mod naive;

pub use kmp::kmp;
pub use naive::naive;

#[cfg(test)]
mod tests {
    use super::*;

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

    fn search_02<'a, F, I>(f: F)
    where
        F: Fn(&'a str, &'a str) -> I,
        I: Iterator<Item = usize> + 'a,
    {
        let x = "aaaaa";
        let p = "aa";
        let result: Vec<usize> = f(x, p).collect();
        assert_eq!(result, vec![0, 1, 2, 3]);
    }

    fn search_03<'a, F, I>(f: F)
    where
        F: Fn(&'a str, &'a str) -> I,
        I: Iterator<Item = usize> + 'a,
    {
        let x = "hello";
        let p = "ll";
        let result: Vec<usize> = f(x, p).collect();
        assert_eq!(result, vec![2]);
    }

    macro_rules! search_tests {
        ($($test_name:ident: $search_fn:expr,)*) => {
            $(
                #[test]
                fn $test_name() {
                    search_01($search_fn);
                    search_02($search_fn);
                    search_03($search_fn);
                }
            )*
        }
    }

    search_tests! {
        naive_tests: naive,
        kmp_tests: kmp,
    }
}
