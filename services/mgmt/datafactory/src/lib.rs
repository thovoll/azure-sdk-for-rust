#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2018-06")]
pub mod package_2018_06;
#[cfg(all(feature = "package-2018-06", not(feature = "no-default-version")))]
pub use package_2018_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2017-09-preview")]
pub mod package_2017_09_preview;
#[cfg(all(feature = "package-2017-09-preview", not(feature = "no-default-version")))]
pub use package_2017_09_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
