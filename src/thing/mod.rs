mod thing_array;
mod thing_func;
mod thing_type;

pub use thing_array::ThingArray;
pub use thing_func::ThingFunc;
pub use thing_type::ThingType;

use std::fmt::{Display, Formatter};

impl Display for ThingFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<T> Display for ThingType<T> {
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
