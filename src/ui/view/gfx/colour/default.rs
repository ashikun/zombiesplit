//! The default zombiesplit palette.
//!
//! Ideally this would be the `Default` implementation for the palette, but the layout of types
//! and serde requirements make this surprisingly complicated in practice.  Plus, the palette
//! is constant.
//!
//! All the default colours are EGA ones, for that homely retro feel.

use ugly::colour::EGA;

use super::{bg, fg, Palette};

/// The default colour palette.
pub const PALETTE: Palette = Palette { fg: FG, bg: BG };

/// The default foreground colour palette.
pub const FG: fg::Map = fg::Map {
    editor: EGA.bright.cyan,
    field_editor: EGA.bright.white,
    header: EGA.bright.red,
    normal: EGA.dark.white,
    status: EGA.dark.black,
    name: fg::PositionMap {
        coming: EGA.dark.white,
        cursor: EGA.bright.magenta,
        done: EGA.bright.white,
    },
    pace: fg::PaceMap {
        inconclusive: EGA.bright.white,
        behind: EGA.bright.red,
        behind_but_gaining: EGA.dark.red,
        ahead_but_losing: EGA.dark.green,
        ahead: EGA.bright.green,
        personal_best: EGA.bright.yellow,
    },
};

/// The default background map.
pub const BG: bg::Map = bg::Map {
    window: EGA.dark.black,
    editor: EGA.dark.blue,
    field_editor: EGA.dark.cyan,
    status: EGA.dark.white,
};
