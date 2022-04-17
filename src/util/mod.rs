pub mod fuzzy_comparison;

pub trait Defaultable {
    fn default() -> Self
    where
        Self: Sized;
}
