#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2018-09")]
pub mod package_2018_09;
#[cfg(all(feature = "package-2018-09", not(feature = "no-default-version")))]
pub use package_2018_09::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2016-05")]
pub mod package_2016_05;
#[cfg(all(feature = "package-2016-05", not(feature = "no-default-version")))]
pub use package_2016_05::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2015-05-preview")]
pub mod package_2015_05_preview;
#[cfg(all(feature = "package-2015-05-preview", not(feature = "no-default-version")))]
pub use package_2015_05_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
