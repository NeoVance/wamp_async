use crate::{
    WampDict,
    Arg
};
use crate::options::option::{
    OptionBuilder,
    WampOption,
};

/// Base struct for storing WampDict value
pub struct CancelOptions(Option<WampDict>);

/// Provides functions for adding defined options to the WampDict
impl CancelOptions {
    /// Add an option for pattern matching the topic of the subscription
    pub fn with_mode(&self, mode_option: &str) -> Self {
        self.with_option(WampOption::CancelOption("mode".to_owned(), Arg::String(mode_option.to_owned())))
    }
}

/// Add base OptionBuilder functionality
impl OptionBuilder for CancelOptions {
    /// Build a new CancelOptions from a provided Option<WampDict>
    fn create(options: Option<WampDict>) -> Self where Self: OptionBuilder + Sized {
        Self(options)
    }

    /// Return the WampDict being operated on and stored by CancelOptions
    fn get_dict(&self) -> Option<WampDict> {
        self.0.clone()
    }
}

/// Default
impl Default for CancelOptions {
    /// Create a new empty CancelOptions
    fn default() -> Self {
        Self::create(Self::empty().with_mode("killnowait").get_dict())
    }
}
