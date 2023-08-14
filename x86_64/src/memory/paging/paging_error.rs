/// Type that compress all Paging errors
pub enum PagingError {
    /// Happens when we try to crate a new page with an address that is not aligned.
    InvalidAlign,
}
