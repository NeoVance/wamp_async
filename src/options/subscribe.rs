use crate::{
    WampDict,
    Arg
};
use crate::options::option::{
    OptionBuilder,
    WampOption,
};

/// Base struct for storing WampDict value
pub struct SubscribeOptions(Option<WampDict>);

/// Provides functions for adding defined options to the WampDict
impl SubscribeOptions {
    /// Add an option for pattern matching the topic of the subscription
    pub fn with_match(&self, match_option: &str) -> Self {
        self.with_option(WampOption::SubscribeOption("match".to_owned(), Arg::String(match_option.to_owned())))
    }
}

/// Add base OptionBuilder functionality
impl OptionBuilder for SubscribeOptions {
    /// Build a new SubscribeOptions from a provided Option<WampDict>
    fn create(options: Option<WampDict>) -> Self where Self: OptionBuilder + Sized {
        Self(options)
    }

    /// Return the WampDict being operated on and stored by SubscribeOptions
    fn get_dict(&self) -> Option<WampDict> {
        self.0.clone()
    }
}

/// Default
impl Default for SubscribeOptions {
    /// Create a new empty SubscribeOptions
    fn default() -> Self {
        Self::empty()
    }
}
