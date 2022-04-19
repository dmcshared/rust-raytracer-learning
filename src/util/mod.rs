use std::sync::Arc;

pub mod fuzzy_comparison;

pub trait NewAsArc {
    fn as_arc(self) -> Arc<Self>;
}

impl<T> NewAsArc for T {
    fn as_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}
