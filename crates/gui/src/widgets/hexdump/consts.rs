use iced_graphics::triangle::Vertex2D;
use iced_graphics::Vector;
use std::ops::Range;

pub const CURSOR_MESH: (&[Vertex2D], &[u32]) = (
    &[
        Vertex2D {
            position: [0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex2D {
            position: [2.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex2D {
            position: [0.0, 6.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex2D {
            position: [2.0, 6.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex2D {
            position: [2.0, 4.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex2D {
            position: [24.0, 4.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex2D {
            position: [24.0, 6.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex2D {
            position: [24.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex2D {
            position: [26.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex2D {
            position: [26.0, 6.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
    ],
    &[0, 1, 3, 0, 2, 3, 4, 5, 6, 4, 3, 6, 7, 8, 9, 7, 6, 9],
);

pub const CURSOR_RIGHT_VERTEX: ([usize; 3], [usize; 2]) = ([5, 6, 7], [8, 9]);

pub const CURSOR_PADDING: f32 = 4.0;

/// Spacing between each row in an [`Hexview`].
///
/// [`Hexview`]: struct.Heview.html
pub const LINE_SPACING: f32 = 8.0;

/// General margins used in an [`Hexview`].
///
/// This constant is used both for padding the text and also for spacing the
/// section separators.
///
/// [`Hexview`]: struct.Heview.html
pub const MARGINS: Vector = Vector::new(10.0, 10.0);
pub const HEX_CHARS: &[u8] = b"0123456789ABCDEF\
                           0123456789ABCDEF";
pub const OFFSET_REFERENCE: &str = "00000000";
pub const BYTES_HEADER: &str = "00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F \
                                    10 11 12 13 14 15 16 17 18 19 1A 1B 1C 1D 1E 1F";
pub const ASCII_RANGE: Range<u8> = 32..128;
