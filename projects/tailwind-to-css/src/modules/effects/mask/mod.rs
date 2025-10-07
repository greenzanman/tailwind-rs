
use super::*;
pub(crate) mod mask_image;

#[inline]
pub(crate) fn mask_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary, neg: Negative) -> Result<Box<dyn TailwindInstance>> {
    let out = match pattern {
        // This is the main utility that sets the `mask-image` property.
        // It handles gradients, `none`, and arbitrary values.
        ["none"] | ["linear" | "radial" | "conic", ..] => {
            TailwindMaskImage::parse(pattern, arbitrary, neg)?.boxed()
        }
        [] => { // Handles arbitrary values like `mask-[radial-gradient(...)]`
            if arbitrary.as_str().contains("gradient(") {
                TailwindMaskImage::parse(pattern, arbitrary, neg)?.boxed()
            } else {
                // In a full implementation, this would route to other mask utilities
                // like mask-size, mask-mode, etc.
                return syntax_error!("Unsupported arbitrary mask value: {}", arbitrary.as_str());
            }
        }
        // TODO: In a full implementation, add other mask utilities here, for example:
        // ["composite", rest @ ..] => TailwindMaskComposite::parse(rest, arbitrary)?.boxed(),
        // ["repeat", rest @ ..] => TailwindMaskRepeat::parse(rest, arbitrary)?.boxed(),
        _ => return syntax_error!("Unknown mask pattern: mask-{}", pattern.join("-")),
    };
    Ok(out)
}