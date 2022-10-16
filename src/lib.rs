// #![warn(missing_docs)] FIXME

mod error;
mod has_channel;
mod midi;
pub mod param;

pub use error::RismidiError;
pub use has_channel::HasChannel;
pub use midi::{constants::*, MidiChannel};
pub use param::OptionalMidiChannelParam;
