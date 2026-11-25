use core::num::NonZeroU32;
use rs_format::{Limits, Position};

#[test]
fn config_defaults() {
    assert_eq!(
        Limits::default(),
        Limits {
            max_depth: 128,
            max_document_bytes: 16_777_216,
            max_token_bytes: usize::try_from(1_048_576_u32).unwrap_or(usize::MAX),
            max_expansion: 33_554_432,
        }
    );
}

#[test]
fn a_line_and_column_do_not_imply_a_byte_offset() {
    let location = Position {
        offset: None,
        line: NonZeroU32::new(1),
        column: NonZeroU32::new(5),
    };
    assert_ne!(
        location,
        Position {
            offset: Some(0),
            ..location
        }
    );
}
