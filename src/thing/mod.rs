mod thing_array;
mod thing_func;
mod thing_type;

pub use thing_array::ThingArray;
pub use thing_func::RecordIdFunc;
pub use thing_type::RecordIdType;

use std::fmt::{Display, Formatter};

impl Display for RecordIdFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<T> Display for RecordIdType<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.thing)
    }
}

impl Display for ThingArray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}
