use serde::export::Formatter;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Error<T: Display + Debug> {
    pub kind: T,
    pub msg: String,
    pub inner: Option<Box<dyn std::error::Error>>,
}

impl<T: Display + Debug> Display for Error<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut msg = format!("{}: {}", self.kind, self.msg);

        if let Some(ref inner) = self.inner {
            msg = format!("{}\nInner error: {}", msg, inner);
        }

        f.write_str(&msg)
    }
}

impl<T: Display + Debug> std::error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.as_ref().map(|e| e.as_ref())
    }
}
