//! Utility definitions.

/// A model element with an attached ID.
pub struct WithID<T> {
    /// The identifier.
    pub id: i64,
    /// The item itself.
    pub item: T,
}

impl<T> WithID<T> {
    /// Consumes this with-ID item and returns a new one with the same ID,
    /// but an item retrieved by mapping `f` over the current item.
    pub fn map_item<T2>(self, f: impl FnOnce(T) -> T2) -> WithID<T2> {
        WithID {
            id: self.id,
            item: f(self.item),
        }
    }

    /// Consumes this with-ID item and returns a new one with the same ID,
    /// but the given `item`.
    pub fn with_item<T2>(self, item: T2) -> WithID<T2> {
        self.map_item(|_| item)
    }
}
