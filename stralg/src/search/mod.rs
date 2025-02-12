pub mod bmh;
pub mod kmp;
pub mod naive;

pub use bmh::bmh;
pub use kmp::kmp;
pub use naive::naive;

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;

    type SearchFn<'a> = fn(&'a str, &'a str) -> Box<dyn Iterator<Item = usize> + 'a>;

    fn search_empty_x(f: SearchFn) {
        let x = "";
        let p = "abr";
        let result: Vec<usize> = f(x, p).collect();
        assert_eq!(result, vec![]);
    }

    fn search_empty_p(f: SearchFn) {
        let x = "abracadabra";
        let p = "";
        let result: Vec<usize> = f(x, p).collect();
        assert_eq!(result, vec![]);
    }

    fn search_01(f: SearchFn) {
        let x = "abracadabra";
        let p = "abr";
        let result: Vec<usize> = f(x, p).collect();
        assert_eq!(result, vec![0, 7]);
    }

    fn search_02(f: SearchFn) {
        let x = "aaaaa";
        let p = "aa";
        let result: Vec<usize> = f(x, p).collect();
        assert_eq!(result, vec![0, 1, 2, 3]);
    }

    fn search_03(f: SearchFn) {
        let x = "hello";
        let p = "ll";
        let result: Vec<usize> = f(x, p).collect();
        assert_eq!(result, vec![2]);
    }

    fn search_when_p_is_longer(f: SearchFn) {
        let x = "abracadabra";
        let p = "abracadabracadabra";
        let result: Vec<usize> = f(x, p).collect();
        assert_eq!(result, vec![]);
    }

    fn search_when_p_has_characters_not_in_x(f: SearchFn) {
        let x = "abracadabra";
        let p = "abrx";
        let result: Vec<usize> = f(x, p).collect();
        assert_eq!(result, vec![]);
    }

    macro_rules! search_tests {
        ($($test_name:ident: $search_fn:expr,)*) => {
            $(
                paste! {
                    #[test]
                    fn [<$test_name _search_empty_x>]() {
                        search_empty_x($search_fn);
                    }

                    #[test]
                    fn [<$test_name _search_empty_p>]() {
                        search_empty_p($search_fn);
                    }

                    #[test]
                    fn [<$test_name _search_01>]() {
                        search_01($search_fn);
                    }

                    #[test]
                    fn [<$test_name _search_02>]() {
                        search_02($search_fn);
                    }

                    #[test]
                    fn [<$test_name _search_03>]() {
                        search_03($search_fn);
                    }

                    #[test]
                    fn [<$test_name _search_when_p_is_longer>]() {
                        search_when_p_is_longer($search_fn);
                    }

                    #[test]
                    fn [<$test_name _search_when_p_has_characters_not_in_x>]() {
                        search_when_p_has_characters_not_in_x($search_fn);
                    }
                }
            )*
        }
    }

    search_tests! {
        naive_tests: naive,
        kmp_tests: kmp,
    }

    search_tests! {
        bmh_tests: bmh,
    }
}
