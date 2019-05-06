pub use self::border::*;
pub use self::brush::*;
pub use self::dirty_size::*;
pub use self::point::*;
pub use self::rect::*;
pub use self::thickness::*;

pub use orbgl_api::Color as Color;

mod border;
mod brush;
mod dirty_size;
mod point;
pub mod prelude;
mod rect;
mod spacer;
mod thickness;

#[cfg(test)]
mod tests;