#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "no-default-version")))]
pub use package_2021_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-09-01")]
pub mod package_2018_09_01;
#[cfg(all(feature = "package-2018-09-01", not(feature = "no-default-version")))]
pub use package_2018_09_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2017-07-01-privatepreview")]
pub mod package_2017_07_01_privatepreview;
#[cfg(all(feature = "package-2017-07-01-privatepreview", not(feature = "no-default-version")))]
pub use package_2017_07_01_privatepreview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
