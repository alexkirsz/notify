//! Configuration types

use std::time::Duration;

/// Indicates whether only the provided directory or its sub-directories as well should be watched
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RecursiveMode {
    /// Watch all sub-directories as well, including directories created after installing the watch
    Recursive,

    /// Watch only the provided directory
    NonRecursive,
}

impl RecursiveMode {
    pub(crate) fn is_recursive(&self) -> bool {
        match *self {
            RecursiveMode::Recursive => true,
            RecursiveMode::NonRecursive => false,
        }
    }
}

/// Runtime configuration items for watchers.
///
/// See the [`Watcher::configure`](./trait.Watcher.html#tymethod.configure) method for usage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Config {
    /// Enable or disable emitting precise event classification.
    ///
    /// Applicable to all watchers.
    ///
    /// When enabled, events are emitted with a `kind` set to as much precision about what kind of
    /// event they are as the backend is capable of providing. When disabled (default), events are
    /// instead emitted as `EventKind::Any`.
    PreciseEvents(bool),

    /// Enable or disable emitting `OngoingWrite` events.
    ///
    /// Applicable to debounced watchers only.
    ///
    /// When enabled, partial write events that are received after a `Modify(Data)` notice but
    /// before the end of a debouncing period (and the emission of a `Modify(Data)` event) are
    /// passed through as `Modify(Data)` events with an `Ongoing` flag. These events are still
    /// debounced, but at a lower (configurable) interval than the debouncing interval.
    ///
    /// To enable, provide `Some(Duration)`. To disable, provide `None`.
    ///
    /// # Errors
    ///
    /// - `InvalidConfigValue` if the interval provided is higher than the debounce interval.
    OngoingWrites(Option<Duration>),
}
