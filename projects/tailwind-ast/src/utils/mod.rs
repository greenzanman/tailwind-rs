use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{char, digit1, hex_digit1},
    combinator::{map_res, not, opt, recognize, peek},
    sequence::{preceded, terminated, tuple},
    IResult,
};
use std::str::FromStr;
#[cfg(test)]
mod tests;

/// `\d+`
pub fn parse_integer<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    map_res(recognize(digit1), str::parse)(input)
}
///
pub fn parse_i_px_maybe<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
{
    let (rest, (i, _)) = tuple((parse_integer, opt(tag("px"))))(input)?;
    Ok((rest, i))
}

/// `\d+\.\d+`
pub fn parse_f32(input: &str) -> IResult<&str, f32> {
    let float1 = tuple((digit1, opt(tuple((tag("."), digit1)))));
    map_res(recognize(float1), str::parse)(input)
}
///
pub fn parse_f_percent(input: &str) -> IResult<&str, f32> {
    let (rest, (f, _)) = tuple((parse_f32, char('%')))(input)?;
    Ok((rest, f))
}

/// `\d+\/\d+`
pub fn parse_fraction(input: &str) -> IResult<&str, (usize, usize)> {
    let (rest, (a, _, b)) = tuple((parse_integer, tag("/"), parse_integer))(input)?;
    Ok((rest, (a, b)))
}

/// 100(/50)?
#[inline]
pub fn parse_fraction_maybe(input: &str) -> IResult<&str, (usize, Option<usize>)> {
    let (rest, (a, b)) = tuple((parse_integer, opt(tuple((tag("/"), parse_integer)))))(input)?;
    Ok((rest, (a, b.map(|s| s.1))))
}

/// --- Hex Color Parser Implementation ---

/// Parses a CSS hex color string like #FFF, #c1c1c1, or #80808080.
///
/// It handles 3, 4, 6, and 8-digit hexadecimal formats, with or without
/// an alpha channel, as specified by the CSS Color Module Level 4.
pub fn parse_color_hex(input: &str) -> IResult<&str, Color> {
    preceded(
        tag("#"),
        // `alt` tries parsers in order, so we try longest first to avoid ambiguity.
        terminated(
            alt((parse_hex8, parse_hex6, parse_hex4, parse_hex3)),
            peek(not(hex_digit1)),
        ),
    )(input)
}

/// Represents an RGBA color.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Color {
    /// The red component of the color.
    pub r: u8,
    /// The green component of the color.
    pub g: u8,
    /// The blue component of the color.
    pub b: u8,
    /// The alpha component of the color.
    pub a: u8,
}


fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

/// Parses two hex characters into a u8 (e.g., "FF" -> 255).
fn hex_pair(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, is_hex_digit),
        |s| u8::from_str_radix(s, 16),
    )(input)
}

/// Parses one hex character and expands it to two (e.g., "F" -> "FF" -> 255).
fn hex_short(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(1, 1, is_hex_digit),
        |s| u8::from_str_radix(&format!("{}{}", s, s), 16),
    )(input)
}

/// Parses an 8-digit hex color: #RRGGBBAA
fn parse_hex8(input: &str) -> IResult<&str, Color> {
    let (input, (r, g, b, a)) = tuple((hex_pair, hex_pair, hex_pair, hex_pair))(input)?;
    Ok((input, Color { r, g, b, a }))
}

/// Parses a 6-digit hex color: #RRGGBB
fn parse_hex6(input: &str) -> IResult<&str, Color> {
    let (input, (r, g, b)) = tuple((hex_pair, hex_pair, hex_pair))(input)?;
    Ok((input, Color { r, g, b, a: 255 }))
}

/// Parses a 4-digit hex color: #RGBA
fn parse_hex4(input: &str) -> IResult<&str, Color> {
    let (input, (r, g, b, a)) = tuple((hex_short, hex_short, hex_short, hex_short))(input)?;
    Ok((input, Color { r, g, b, a }))
}

/// Parses a 3-digit hex color: #RGB
fn parse_hex3(input: &str) -> IResult<&str, Color> {
    let (input, (r, g, b)) = tuple((hex_short, hex_short, hex_short))(input)?;
    Ok((input, Color { r, g, b, a: 255 }))
}