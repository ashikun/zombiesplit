//! Functionality for dealing with carries between time fields.

/// Represents the result of a time computation that has generated carry.
pub struct Carry<T> {
    /// The value for which carry has been computed.
    pub value: T,
    /// The carry amount.
    pub carry: u32,
    /// The original input.
    pub original: u32,
}

impl Carry<u32> {
    /// Constructs a carry by dividing `original` by `divisor`.
    #[must_use]
    pub fn from_division(original: u32, divisor: u32) -> Self {
        let value = original % divisor;
        Self {
            value,
            carry: (original - value) / divisor,
            original,
        }
    }
}

impl<T> Carry<T> {
    /// Destructively maps `f` across this carry.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Carry<U> {
        Carry {
            value: f(self.value),
            carry: self.carry,
            original: self.original,
        }
    }
}
