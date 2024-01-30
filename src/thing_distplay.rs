use crate::{ThingFunc, ThingType};
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
