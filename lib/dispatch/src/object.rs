#[allow(drop_bounds)] // All dispatch objects must be dropped (released).
pub trait Object: Drop {}
