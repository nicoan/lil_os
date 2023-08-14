// Marker trait used to limit the generic parameter of the mapper struct to types that implements
// this marker
pub trait FrameSize {}

/// Represents a frame size of 4 KiB
pub struct FrameSize4KiB;
impl FrameSize for FrameSize4KiB {}

/// Represents a (huge) frame size of 2 MiB
pub struct FrameSize2MiB;
impl FrameSize for FrameSize2MiB {}

/// Represents a (huge) frame size of 1 GiB
pub struct FrameSize1GiB;
impl FrameSize for FrameSize1GiB {}
