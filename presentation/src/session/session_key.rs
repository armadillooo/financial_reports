use std::marker::PhantomData;

/// Sessionに保存された各要素ごとのKey
#[derive(Debug, Clone)]
pub struct SessionKey<T> {
    pub value: String,
    phantom: PhantomData<T>,
}

impl<T> SessionKey<T> {
    pub const fn new(key: String) -> Self {
        Self {
            value: key,
            phantom: PhantomData,
        }
    }
}
