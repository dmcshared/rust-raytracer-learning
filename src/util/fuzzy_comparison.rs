pub fn f64_fuzzy_eq(left: f64, right: f64) -> bool {
    let epsilon = 0.00001;
    (left - right).abs() < epsilon
}

pub trait FuzzyPartialEq<T> {
    fn fuzzy_eq(self, other: T) -> bool;
}

impl FuzzyPartialEq<f64> for f64 {
    fn fuzzy_eq(self, other: f64) -> bool {
        f64_fuzzy_eq(self, other)
    }
}

#[macro_export]
macro_rules! assert_fuzzy_eq {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                assert!((*left_val).fuzzy_eq(*right_val));
            }
        }
    }};

    ($left:expr, $right:expr, $($arg:tt)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                assert!((*left_val).fuzzy_eq(*right_val), $($arg)?);
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_fuzzy_neq {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                assert!(!(*left_val).fuzzy_eq(*right_val));
            }
        }
    }};

    ($left:expr, $right:expr, $($arg:tt)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                assert!(!(*left_val).fuzzy_eq(*right_val), $($arg)?);
            }
        }
    }};
}
