#[derive(Debug)]
pub struct Pt(pub i16, pub i16);

#[derive(Debug)]
pub enum Stroke {
    Line {
        from: Pt,
        to: Pt,
    },
    Arc {
        center: Pt,
        radius: i16,
        start: i16,
        end: i16,
    },
}

#[derive(Debug)]
pub struct Glyph {
    pub ch: char,
    pub strokes: Vec<Stroke>,
    pub width_restriction: Option<u8>, // 0-100 percentage of width
}