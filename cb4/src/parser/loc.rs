use logos::Span;

///
#[derive(Clone, PartialEq, Eq, Default, Copy)]
#[repr(C)]
pub struct Loc {
    /// Begin of the `Loc` range
    pub begin: u32,

    /// End of the `Loc` range
    pub end: u32,
}

impl Loc {
    /// Converts location to a range
    pub fn to_range(&self) -> std::ops::Range<u32> {
        self.begin..self.end
    }

    pub fn from_span(span: &Span) -> Self {
        Loc {
            begin: span.start as u32,
            end: span.end as u32,
        }
    }
}

impl std::fmt::Debug for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_ /*'*/>) -> Result<(), std::fmt::Error> {
        f.write_str(&format!("{}...{}", self.begin, self.end))
    }
}
