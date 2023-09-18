// Marker trait used to limit the generic parameter of the mapper struct to types that implements
// this marker
pub trait PageSize {}

/// Represents a page (or frame) size of 4 KiB
#[derive(Clone, Copy, Debug)]
pub struct Size4KiB;
impl PageSize for Size4KiB {}

/// Represents a (huge) page (or frame) size of 2 MiB
#[derive(Clone, Copy, Debug)]
pub struct Size2MiB;
impl PageSize for Size2MiB {}

/// Represents a (huge) page (or frame) size of 1 GiB
#[derive(Clone, Copy, Debug)]
pub struct Size1GiB;
impl PageSize for Size1GiB {}
