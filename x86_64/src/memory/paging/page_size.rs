// Marker trait used to limit the generic parameter of the mapper struct to types that implements
// this marker
pub trait PageSize {}

/// Represents a page size of 4 KiB
pub struct PageSize4KiB;
impl PageSize for PageSize4KiB {}

/// Represents a (huge) page size of 2 MiB
pub struct PageSize2MiB;
impl PageSize for PageSize2MiB {}

/// Represents a (huge) page size of 1 GiB
pub struct PageSize1GiB;
impl PageSize for PageSize1GiB {}
