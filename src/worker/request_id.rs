use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct RequestId<T> {
    id: u32,
    _type: PhantomData<*const T>
}

impl<T> RequestId<T> {
    pub fn new(id: u32) -> RequestId<T> {
        RequestId {
            id,
            _type: PhantomData
        }
    }
    
    pub fn to_string(&self) -> String {
        format!("RequestId: {}", self.id)
    }
}