use super::*;

pub(crate) mod ring_color;
pub(crate) mod ring_inset;
pub(crate) mod ring_offset_color;
pub(crate) mod ring_offset_width;
pub(crate) mod ring_width;

/// Adaptor function to parse all ring (solid box-shadow) utilities.
pub(crate) fn ring_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let out = match pattern {
        // Handles `ring-inset`
        ["inset"] => TailwindRingInset {}.boxed(),
        // Handles `ring-offset-*`
        // Differentiates between width and color for the offset.
        ["offset", rest @ ..] => {
            // Check if the first part of `rest` is numeric, which implies a width.
            // This handles `ring-offset-2`, `ring-offset-[4px]`, etc.
            if let Some(first) = rest.first() {
                if first.chars().all(char::is_numeric) {
                    return Ok(TailwindRingOffsetWidth::parse(rest, arbitrary)?.boxed());
                }
            }
            // If it's not a numeric width, parse it as an offset color.
            // This handles `ring-offset-blue-500`, etc.
            TailwindRingOffsetColor::parse(rest, arbitrary)?.boxed()
        }

        // Handles explicit colors like `ring-black`, `ring-white`
        ["black" | "white" | "inherit" | "current" | "transparent" | "revert"] =>
            TailwindRingColor::parse(pattern, arbitrary)?.boxed(),

        // --- Flexible Parsing for Width and Color ---

        // Handles `ring-[2px]` or `ring-[theme(spacing.2)]`
        [] if arbitrary.is_some() => TailwindRingWidth::from(arbitrary).boxed(),

        // Handles the base `ring` class (defaults to a width)
        [] => {
            let default_kind = UnitValue::Keyword("<DEFAULT>".to_string());
            TailwindRingWidth::from(default_kind).boxed()
        }

        // Handles shorthand like `ring-2` or `ring-blue-500`
        [n] => resolve1(n)?,

        // Fallback to parsing as a color for patterns like `ring-blue-500`
        _ => TailwindRingColor::parse(pattern, arbitrary)?.boxed(),
    };
    Ok(out)
}

/// Helper to resolve a single segment like "2" or "blue-500".
fn resolve1(n: &str) -> Result<Box<dyn TailwindInstance>> {
    let a = TailwindArbitrary::from(n);

    // If it starts with a number, parse as a width (e.g., "2", "3.5")
    if n.starts_with(|c: char| c.is_numeric()) {
        // Try parsing as a length first (e.g., "1.5"), then fallback to integer ("2")
        return Ok(resolve1_length(&a).or_else(|_| resolve1_unit(&a))?.boxed());
    }

    // If it starts with '#', it's a hex color code.
    if n.starts_with('#') {
        return Ok(resolve1_color(&a)?.boxed());
    }
    
    // Otherwise, assume it's a themed color (e.g., "blue-500").
    Ok(TailwindRingColor::from(TailwindColor::Themed(n.to_string(), 0)).boxed())
}

/// Helper to parse an arbitrary value as a length-based width.
fn resolve1_length(a: &TailwindArbitrary) -> Result<ring_width::TailwindRingWidth> {
    Ok(ring_width::TailwindRingWidth::from(a.as_length()?))
}

/// Helper to parse an arbitrary value as an integer-based width.
fn resolve1_unit(a: &TailwindArbitrary) -> Result<ring_width::TailwindRingWidth> {
    Ok(ring_width::TailwindRingWidth::from(a.as_integer()?))
}

/// Helper to parse an arbitrary value as a color.
fn resolve1_color(a: &TailwindArbitrary) -> Result<ring_color::TailwindRingColor> {
    Ok(ring_color::TailwindRingColor::from(a.as_color()?))
}