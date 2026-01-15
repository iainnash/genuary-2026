use crate::types::{Pt, Stroke, Glyph};
use std::collections::HashMap;

macro_rules! glyphs {
    (
        $(
            $ch:literal {
                $($body:tt)*
            }
        )*
    ) => {{
        let mut glyphs = HashMap::new();
        $(
            let (strokes, width_restriction) = glyphs!(@parse_strokes $($body)*);
            let glyph = Glyph {
                ch: $ch,
                strokes,
                width_restriction,
            };
            glyphs.insert(glyph.ch, glyph);
        )*
        glyphs
    }};

    // top level recursive parsing
    (@parse_strokes $($tt:tt)*) => {{
        let mut strokes_vec: Vec<Stroke> = Vec::new();
        let mut width_restriction: Option<u8> = None;
        glyphs!(@strokes strokes_vec, width_restriction, $($tt)*);
        (strokes_vec, width_restriction)
    }};

    // Base case: no more strokes
    (@strokes $out:ident, $width:ident, ) => {};

    // lines grammar
    (@strokes $out:ident, $width:ident,
        line ($x1:expr, $y1:expr) -> ($x2:expr, $y2:expr)
        $($rest:tt)*
    ) => {{
        $out.push(Stroke::Line {
            from: Pt($x1, $y1),
            to: Pt($x2, $y2),
        });
        glyphs!(@strokes $out, $width, $($rest)*);
    }};

    // arcs grammar
    (@strokes $out:ident, $width:ident,
        arc ($x:expr, $y:expr) radius ($r:expr) from ($start:expr) -> ($end:expr)
        $($rest:tt)*
    ) => {{
        $out.push(Stroke::Arc {
            center: Pt($x, $y),
            radius: $r,
            start: $start,
            end: $end,
        });
        glyphs!(@strokes $out, $width, $($rest)*);
    }};

    // restrict_width grammar
    (@strokes $out:ident, $width:ident,
        restrict_width ($w:expr)
        $($rest:tt)*
    ) => {{
        $width = Some($w as u8);
        glyphs!(@strokes $out, $width, $($rest)*);
    }};
}

pub fn define_glyphs() -> HashMap<char, Glyph> {
    let glyphs = glyphs! {
        // Space
        ' ' {
            // Empty glyph
        }

        // Letter g - arc with a line extending from the bottom right
        'g' {
            arc (50, 50) radius (50) from (0) -> (360)
            arc (50, 10) radius (50) from (240) -> (390)
        }

        // Letter e - 3/4 circle with a horizontal line
        'e' {
            arc (50, 50) radius (50) from (0) -> (270)
            line (50, 50) -> (100, 50)
        }

        // Letter n - vertical line, half circle, and another vertical line
        'n' {
            line (0, 0) -> (0, 100)
            arc (50, 50) radius (50) from (0) -> (180)
            line (100, 0) -> (100, 50)
        }

        // Letter u - vertical line, half circle at bottom, and another vertical line
        'u' {
            line (0, 100) -> (0, 50)
            arc (50, 50) radius (50) from (180) -> (360)
            line (100, 100) -> (100, 50)
        }

        // Letter r - vertical line with a quarter circle at top right
        'r' {
            line (0, 0) -> (0, 100)
            arc (0, 50) radius (50) from (0) -> (90)
            restrict_width (10)
        }

        // Letter a - circle with a vertical line on the right
        'a' {
            arc (50, 50) radius (50) from (0) -> (360)
            line (100, 0) -> (100, 100)
        }

        // Letter y - V shape with a stem
        'y' {
            line (0, 100) -> (50, 50)
            line (50, 50) -> (100, 100)
            line (50, 50) -> (50, 0)
        }

        // Letter b
        'b' {
            line (0, 0) -> (0, 100)
            arc (50, 25) radius (50) from (270) -> (90)
            arc (50, 75) radius (50) from (270) -> (90)
        }

        // Letter c
        'c' {
            arc (50, 50) radius (50) from (45) -> (315)
            restrict_width (70)
        }

        // Letter d
        'd' {
            line (100, 0) -> (100, 100)
            arc (50, 50) radius (50) from (90) -> (270)
        }

        // Letter f
        'f' {
            line (25, 0) -> (25, 100)
            line (25, 100) -> (75, 100)
            line (25, 50) -> (75, 50)
            restrict_width (75)
        }

        // Letter h
        'h' {
            line (0, 0) -> (0, 100)
            line (100, 0) -> (100, 100)
            line (0, 50) -> (100, 50)
        }

        // Letter i
        'i' {
            line (50, 0) -> (50, 100)
            arc (50, 100) radius (5) from (0) -> (360)
            restrict_width (30)
        }

        // Letter j
        'j' {
            line (75, 0) -> (75, 75)
            arc (50, 25) radius (25) from (0) -> (180)
            arc (75, 100) radius (5) from (0) -> (360)
            restrict_width (75)
        }

        // Letter k
        'k' {
            line (0, 0) -> (0, 100)
            line (0, 50) -> (100, 100)
            line (0, 50) -> (100, 0)
            restrict_width (80)
        }

        // Letter l
        'l' {
            line (50, 0) -> (50, 100)
            restrict_width (30)
        }

        // Letter m
        'm' {
            line (0, 0) -> (0, 100)
            line (50, 0) -> (50, 100)
            line (100, 0) -> (100, 100)
            arc (25, 100) radius (25) from (180) -> (0)
            arc (75, 100) radius (25) from (180) -> (0)
        }

        // Letter o
        'o' {
            arc (50, 50) radius (50) from (0) -> (360)
            restrict_width (90)
        }

        // Letter p
        'p' {
            line (0, 0) -> (0, 100)
            arc (50, 75) radius (50) from (270) -> (90)
            restrict_width (80)
        }

        // Letter q
        'q' {
            arc (50, 50) radius (50) from (0) -> (360)
            line (70, 30) -> (100, 0)
        }

        // Letter s
        's' {
            arc (50, 75) radius (25) from (0) -> (270)
            arc (50, 25) radius (25) from (180) -> (90)
            restrict_width (70)
        }

        // Letter t
        't' {
            line (50, 0) -> (50, 100)
            line (25, 100) -> (75, 100)
            restrict_width (70)
        }

        // Letter v
        'v' {
            line (0, 100) -> (50, 0)
            line (50, 0) -> (100, 100)
        }

        // Letter w
        'w' {
            line (0, 100) -> (25, 0)
            line (25, 0) -> (50, 50)
            line (50, 50) -> (75, 0)
            line (75, 0) -> (100, 100)
        }

        // Letter x
        'x' {
            line (0, 0) -> (100, 100)
            line (0, 100) -> (100, 0)
        }

        // Letter z
        'z' {
            line (0, 100) -> (100, 100)
            line (100, 100) -> (0, 0)
            line (0, 0) -> (100, 0)
            restrict_width (80)
        }

        // Number 0
        '0' {
            arc (50, 50) radius (50) from (0) -> (360)
            restrict_width (80)
        }

        // Number 1
        '1' {
            line (50, 0) -> (50, 100)
            restrict_width (30)
        }

        // Number 2
        '2' {
            arc (50, 75) radius (50) from (0) -> (270)
            line (50, 75) -> (0, 0)
            restrict_width (80)
        }
    };

    glyphs
}