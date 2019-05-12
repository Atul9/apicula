//! Load Nitro files.
//!
//! Nitro is the SDK used for many Nintendo DS games. These modules parse
//! the binary format for Nitro files into domain objects and provide other
//! tools specific to these formats.
//!
//! Partial documentation on these formats can be found here.
//!
//! * <http://llref.emutalk.net/docs/?file=xml/bmd0.xml>
//! * <http://llref.emutalk.net/docs/?file=xml/btx0.xml>
//! * <http://llref.emutalk.net/docs/?file=xml/bca0.xml>
//!
//! The code in this module should be more complete.

pub mod animation;
pub mod container;
mod info_block;
pub mod model;
pub mod name;
pub mod pattern;
pub mod render_cmds;
mod rotation;
pub mod tex;

pub use self::animation::Animation;
pub use self::container::Container;
pub use self::model::Model;
pub use self::name::Name;
pub use self::pattern::Pattern;
pub use self::tex::Palette;
pub use self::tex::Texture;
