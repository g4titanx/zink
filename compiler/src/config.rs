//! Zink compiler configuration.

#[cfg(feature = "cli")]
use ccli::clap;

/// Zink compiler configuration.
#[derive(Debug, Default)]
#[cfg_attr(feature = "cli", derive(clap::Parser))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Config {
    /// If enable dispatcher.
    #[cfg_attr(feature = "cli", clap(long))]
    pub dispatcher: bool,
}

impl Config {
    /// With dispatcher value.
    pub fn dispatcher(mut self, dispatcher: bool) -> Self {
        self.dispatcher = dispatcher;
        self
    }
}
