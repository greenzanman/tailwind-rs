use super::*;

pub(crate) mod font_family;
pub(crate) mod font_size;
pub(crate) mod font_smoothing;
pub(crate) mod font_style;
pub(crate) mod font_variant_numeric;
pub(crate) mod font_weight;

pub fn font_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let out = match pattern {
        // https://tailwindcss.com/docs/float
        ["thin"] => TailwindFontWeight::THIN.boxed(),
        ["extralight"] => TailwindFontWeight::EXTRA_LIGHT.boxed(),
        ["light"] => TailwindFontWeight::LIGHT.boxed(),
        ["normal"] => TailwindFontWeight::NORMAL.boxed(),
        ["medium"] => TailwindFontWeight::MEDIUM.boxed(),
        ["semibold"] => TailwindFontWeight::SEMI_BOLD.boxed(),
        ["bold"] => TailwindFontWeight::BOLD.boxed(),
        ["extrabold"] => TailwindFontWeight::EXTRA_BOLD.boxed(),
        ["black"] => TailwindFontWeight::BLACK.boxed(),
        // Extended syntax for font-size (usually it's text-{size})
        ["size"] => TailwindFontSize::parse(pattern, arbitrary)?.boxed(),
        ["size", n] => {
            let a = TailwindArbitrary::from(*n);
            TailwindFontSize::parse(pattern, &a)?.boxed()
        },
        // Try parse as font weight if pattern has one segment
        // - will fail if segment is not an int
        [n] => {
            let a = TailwindArbitrary::from(*n);
            maybe_weight(&a)?
        },
        // Try parse as font family if pattern has 1+ segments
        [_, _rest @ ..] => {
            TailwindFontFamily::from(pattern.join("-")).boxed()
        },
        // If no pattern, try parse the arbitrary:
        // 1) try parse as font weight, 
        // 2) if fails, try parse as font family
        [] => {
            maybe_weight(arbitrary)
               .or_else(|_| Ok::<Box<dyn TailwindInstance>, TailwindError>(
                    TailwindFontFamily::from(arbitrary.get_properties()).boxed()
                ))?
        }
    };
    Ok(out)
}

fn maybe_weight(arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
    let w = arbitrary.as_integer()?;
    Ok(TailwindFontWeight::new(w).boxed())
}