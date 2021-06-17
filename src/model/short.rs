//! Type aliases for short names and associated types.

/// Type alias for short names.
pub type Name = String;

/// Type alias for maps from short names to items.
pub type Map<T> = std::collections::HashMap<Name, T>;
