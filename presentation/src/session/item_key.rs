use std::marker::PhantomData;

/// Sessionに保存された各要素ごとのKey
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ItemKey<T> {
    pub value: &'static str,
    phantom: PhantomData<T>,
}

impl<T> ItemKey<T> {
    pub const fn new(key: &'static str) -> Self {
        Self {
            value: key,
            phantom: PhantomData,
        }
    }
}
