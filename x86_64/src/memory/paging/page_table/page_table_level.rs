/// Represents a level of the multilevel Page Table
#[derive(Clone, Copy)]
pub enum PageTableLevel {
    Level1,
    Level2,
    Level3,
    Level4,
}
