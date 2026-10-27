mod geom;
mod map;

pub use geom::{Dir, Point};
pub use map::{Map, Tile};

pub fn hello() -> &'static str {
    "hello from rouge-core"
}