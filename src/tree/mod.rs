pub mod tree;
pub mod misc;
pub mod animation;
pub mod playlist;
pub mod animator;

pub mod prelude {
    pub use crate::tree::tree;
    pub use crate::tree::misc;
    pub use crate::tree::animation;
    pub use crate::tree::playlist;
    pub use crate::tree::animator;
}
