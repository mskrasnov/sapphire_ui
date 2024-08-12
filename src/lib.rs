//! # Sapphire UI Kit
//!
//! Sapphire UI Kit contains new themed components for [Iced](https://iced.rs).
//! *Uses color schemes from [prettygooey](https://github.com/pieterdd/prettygooey)*.
//! Sapphire UI Kit is primarily focused on the regular desktop (Linux, Windows,
//! macOS).
//!
//! ## Structure
//!
//! The Crate is divided into two large parts: [`theme`] and [`widgets`]. The
//! `theme` module contains only a styles, while `widgets` contains implementations
//! of these widgets (as functions). Most functions are wrappers over Iced
//! functions, but with modified styles applied. Sometimes it is allowed to change
//! other non-styles parameters (such as widget sizes, fonts, etc.).
//!
//! Check the [examples](https://github.com/mskrasnov/sapphire_ui/tree/refact/examples)
//! for more details!
//!
//! ## Documentation
//!
//! You can build API documentation using `cargo`:
//!
//! ```no-test
//! cargo doc --no-deps --open
//! ```
//!
//! Human Interface Guidelines, Introduction for beginners and User Manual
//! (`mdbook` is required!):
//!
//! ```no-test
//! cd docs/
//! mdbook serve --open
//! ```
//!
//! ## License
//!
//! Sapphire UI Kit distributed under MIT license.
//!
//! ## Support me
//!
//! Users from Russian and Belarus can use Sberbank: 2202206252335406 ...
//!
//! ... or use [Boosty](https://boosty.to/linux-for-arm/donate) (don't be surprised
//! by the address - at first the page on boosty was for my Linux for ARM project,
//! but in the future I started using it for other projects as well.).

pub mod theme;
pub mod widgets;

pub use theme::ColorName;
pub use theme::WidgetName;

pub use theme::Theme;
