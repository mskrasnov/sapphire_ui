use crate::accent::Accent;
pub use crate::text::*;

#[derive(Clone, Copy)]
pub struct Theme {
    pub accent_color: Accent,
}