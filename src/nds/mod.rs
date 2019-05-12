//! NDS hardware functions.

pub mod decode_texture;
pub mod gpu_cmds;
pub mod texture_formats;
pub mod texture_params;

pub use self::decode_texture::decode_texture;
pub use self::texture_formats::{Alpha, FormatDesc, TextureFormat};
pub use self::texture_params::TextureParams;
