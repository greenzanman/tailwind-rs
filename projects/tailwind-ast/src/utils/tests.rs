use super::*;

#[test]
fn test_isize() {
    assert_eq!(parse_integer("0"), Ok(("", 0isize)));
    assert_eq!(parse_integer("42"), Ok(("", 42isize)));
}

#[test]
fn test_usize() {
    assert_eq!(parse_integer("0"), Ok(("", 0usize)));
    assert_eq!(parse_integer("42"), Ok(("", 42usize)));
}

#[test]
fn test_f32() {
    assert_eq!(parse_f32("0"), Ok(("", 0.0)));
    assert_eq!(parse_f32("42"), Ok(("", 42.0)));
    assert_eq!(parse_f32("99.99"), Ok(("", 99.99)));
}

#[test]
fn test_fraction() {
    assert_eq!(parse_fraction("1/12"), Ok(("", (1, 12))));
    assert_eq!(parse_fraction("12/2"), Ok(("", (12, 2))));
    assert_eq!(parse_fraction("12/24"), Ok(("", (12, 24))));
}

#[test]
fn test_color_hex() {
    // 6-digit
    assert_eq!(
        parse_color_hex("#00ff00"),
        Ok(("", Color { r: 0, g: 255, b: 0, a: 255 }))
    );
    assert_eq!(
        parse_color_hex("#FFD700"),
        Ok(("", Color { r: 255, g: 215, b: 0, a: 255 }))
    );

    // 8-digit
    assert_eq!(
        parse_color_hex("#0000ffcc"),
        Ok(("", Color { r: 0, g: 0, b: 255, a: 204 }))
    );
    assert_eq!(
        parse_color_hex("#80808080"),
        Ok(("", Color { r: 128, g: 128, b: 128, a: 128 }))
    );

    // 3-digit
    assert_eq!(
        parse_color_hex("#123"),
        Ok(("", Color { r: 17, g: 34, b: 51, a: 255 }))
    );
    assert_eq!(
        parse_color_hex("#f0c"),
        Ok(("", Color { r: 255, g: 0, b: 204, a: 255 }))
    );
     assert_eq!(
        parse_color_hex("#FfF"), // mixed case
        Ok(("", Color { r: 255, g: 255, b: 255, a: 255 }))
    );

    // 4-digit
    assert_eq!(
        parse_color_hex("#f0c8"),
        Ok(("", Color { r: 255, g: 0, b: 204, a: 136 }))
    );
    assert_eq!(
        parse_color_hex("#fb0f"),
        Ok(("", Color { r: 255, g: 187, b: 0, a: 255 }))
    );

    // Invalid inputs
    assert!(parse_color_hex("#12345").is_err()); // Invalid length
    assert!(parse_color_hex("123456").is_err());  // Missing '#'
    assert!(parse_color_hex("#12h456").is_err()); // Invalid hex character
}