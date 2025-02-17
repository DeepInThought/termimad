use std::fmt;

use crate::skin::MadSkin;
use crate::line::FmtLine;


/// an internal facility to write just a line of a text
pub struct DisplayableLine<'s, 'l, 'p> {
    pub skin: &'s MadSkin,
    pub line: &'p FmtLine<'l>,
    pub width: Option<usize>, // available width
}

impl<'s, 'l, 'p> DisplayableLine<'s, 'l, 'p> {
    pub fn new(skin: &'s MadSkin, line: &'p FmtLine<'l>, width: Option<usize>) -> DisplayableLine<'s, 'l, 'p> {
        DisplayableLine {
            skin,
            line,
            width,
        }
    }
}

impl fmt::Display for DisplayableLine<'_, '_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.skin.write_fmt_line(f, self.line, self.width, true)?;
        writeln!(f)
    }
}
