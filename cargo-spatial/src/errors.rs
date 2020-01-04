use serde::export::Formatter;
use std::error::Error;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct WrappedError<T: Display + Debug> {
    pub kind: T,
    pub msg: String,
    pub inner: Option<Box<dyn Error>>,
}

impl<T: Display + Debug> Display for WrappedError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut msg = format!("{}: {}", self.kind, self.msg);

        if let Some(ref inner) = self.inner {
            msg = format!("{}\nInner error: {}", msg, inner);
        }

        f.write_str(&msg)
    }
}

impl<T: Display + Debug> Error for WrappedError<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.inner.as_ref().map(|e| e.as_ref())
    }
}
