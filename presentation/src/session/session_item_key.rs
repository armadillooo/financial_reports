use std::marker::PhantomData;

/// Sessionに保存された各要素ごとのKey
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SessionItemKey<T> {
    pub value: &'static str,
    phantom: PhantomData<T>,
}

impl<T> SessionItemKey<T> {
    pub const fn new(key: &'static str) -> Self {
        Self {
            value: key,
            phantom: PhantomData,
        }
    }
}
