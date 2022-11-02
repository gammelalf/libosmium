#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
//! ## Where to start
//!
//! As stated above this crate is for reading osm objects from a file.
//! Therefore, you should start with the [Handler] trait which does exactly that.

mod area;
pub use area::Area;

pub mod handler;
pub use handler::Handler;

mod item;
pub use item::{Item, ItemRef};

mod location;
pub use location::{Location, PRECISION};

mod node;
pub use node::{Node, NodeRef};

pub mod node_ref_list;

mod object;
pub use object::OSMObject;

pub mod tag_list;

mod way;
pub use way::Way;
