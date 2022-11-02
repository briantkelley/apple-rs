/// An interface for safely upcasting Objective-C object instances.
///
/// This is necessary because Rust does not support type inheritance and Objective-C objects
/// cannot generally be represented as fat pointers.
pub trait Upcast<T, U> {
    /// Convert the current object from type `T` to a superclass type `U`.
    fn upcast(from: T) -> U;
}
