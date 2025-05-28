use dioxus::prelude::Props;

#[non_exhaustive]
#[derive(Clone, Props, PartialEq)]
pub struct Config {
    pub base_uri: String,
}
