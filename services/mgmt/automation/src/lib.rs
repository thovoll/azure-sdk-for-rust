#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2015-10")]
pub mod package_2015_10;
#[cfg(all(feature = "package-2015-10", not(feature = "no-default-version")))]
pub use package_2015_10::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2017-05-preview")]
pub mod package_2017_05_preview;
#[cfg(all(feature = "package-2017-05-preview", not(feature = "no-default-version")))]
pub use package_2017_05_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-01-preview")]
pub mod package_2018_01_preview;
#[cfg(all(feature = "package-2018-01-preview", not(feature = "no-default-version")))]
pub use package_2018_01_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-06-preview")]
pub mod package_2018_06_preview;
#[cfg(all(feature = "package-2018-06-preview", not(feature = "no-default-version")))]
pub use package_2018_06_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-06")]
pub mod package_2019_06;
#[cfg(all(feature = "package-2019-06", not(feature = "no-default-version")))]
pub use package_2019_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-01-13-preview")]
pub mod package_2020_01_13_preview;
#[cfg(all(feature = "package-2020-01-13-preview", not(feature = "no-default-version")))]
pub use package_2020_01_13_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-06-22")]
pub mod package_2021_06_22;
#[cfg(all(feature = "package-2021-06-22", not(feature = "no-default-version")))]
pub use package_2021_06_22::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
