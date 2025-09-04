use std::fmt::{Display, Formatter};

/// Tailwind CSS Preflight v4 Style System
/// <https://tailwindcss.com/docs/preflight>
/// https://github.com/tailwindlabs/tailwindcss/blob/88b9f15b65588a87c5b6b13316530b4aecbc1b0b/packages/tailwindcss/preflight.css
#[derive(Clone, Debug)]
pub struct PreflightSystem {
    /// Disables all preflight styles if set to true.
    pub disable: bool,
    /// Resets box-sizing, margins, padding, and borders for all elements.
    pub global_reset: bool,
    /// Applies consistent line-height, font-family, and other root-level styles.
    pub html_base: bool,
    /// Unstyles headings (`h1`-`h6`) to inherit font size and weight.
    pub unstyle_headings: bool,
    /// Resets link colors and text decoration to inherit from their parent.
    pub unstyle_links: bool,
    /// Removes default list styles (`list-style: none`) and resets margin/padding.
    pub unstyle_lists: bool,
    /// Makes images and other replaced elements `display: block`.
    pub block_level_media: bool,
    /// Resets table styles for border-collapse and text-indent.
    pub reset_tables: bool,
    /// Applies a comprehensive reset to form elements like buttons, inputs, and textareas.
    pub reset_forms: bool,
    /// Prevents `hidden` elements from being displayed.
    pub hidden_attribute: bool,

    /// Includes minor typographic styles and element-specific enhancements.
    /// This provides sensible defaults for elements like `<strong>`, `<code>`, `<abbr>`,
    /// and fixes issues such as `<sub>` and `<sup>` affecting line height.
    pub specific_extras: bool,

    /// Includes a collection of hyper-specific browser compatibility fixes.
    /// This is primarily focused on normalizing the appearance of complex form controls,
    /// addressing quirks in WebKit date pickers, search inputs, number input buttons,
    /// and Firefox's focus and invalid input styles.
    pub compatibility_fixes: bool,
    
    /// User-defined custom CSS to be prepended to the preflight styles.
    pub custom: String,
}

impl Default for PreflightSystem {
    /// The default state includes the COMPLETE set of Preflight styles.
    fn default() -> Self {
        Self {
            disable: false,
            global_reset: true,
            html_base: true,
            unstyle_headings: true,
            unstyle_links: true,
            unstyle_lists: true,
            block_level_media: true,
            reset_tables: true,
            reset_forms: true,
            hidden_attribute: true,
            specific_extras: true,
            compatibility_fixes: true,
            custom: String::new(),
        }
    }
}

impl PreflightSystem {
    const GLOBAL_RESET: &'static str = r#"
*,
::after,
::before,
::backdrop,
::file-selector-button {
  box-sizing: border-box; /* 1 */
  margin: 0; /* 2 */
  padding: 0; /* 2 */
  border: 0 solid; /* 3 */
}
"#;

    const HTML_BASE: &'static str = r#"
html,
:host {
  line-height: 1.5;
  -webkit-text-size-adjust: 100%;
  tab-size: 4;
  font-family: --theme(
    --default-font-family,
    ui-sans-serif,
    system-ui,
    sans-serif,
    'Apple Color Emoji',
    'Segoe UI Emoji',
    'Segoe UI Symbol',
    'Noto Color Emoji'
  );
  font-feature-settings: --theme(--default-font-feature-settings, normal);
  font-variation-settings: --theme(--default-font-variation-settings, normal);
  -webkit-tap-highlight-color: transparent;
}
hr {
  height: 0;
  color: inherit;
  border-top-width: 1px;
}
"#;

    const UNSTYLE_HEADINGS: &'static str = r#"
h1,
h2,
h3,
h4,
h5,
h6 {
  font-size: inherit;
  font-weight: inherit;
}
"#;

    const UNSTYLE_LINKS: &'static str = r#"
a {
  color: inherit;
  -webkit-text-decoration: inherit;
  text-decoration: inherit;
}
"#;

    const UNSTYLE_LISTS: &'static str = r#"
ol,
ul,
menu {
  list-style: none;
}
"#;

    const BLOCK_LEVEL_MEDIA: &'static str = r#"
img,
svg,
video,
canvas,
audio,
iframe,
embed,
object {
  display: block;
  vertical-align: middle;
}
img,
video {
  max-width: 100%;
  height: auto;
}
"#;

    const RESET_TABLES: &'static str = r#"
table {
  text-indent: 0;
  border-color: inherit;
  border-collapse: collapse;
}
"#;

    const RESET_FORMS: &'static str = r#"
button,
input,
select,
optgroup,
textarea,
::file-selector-button {
  font: inherit;
  font-feature-settings: inherit;
  font-variation-settings: inherit;
  letter-spacing: inherit;
  color: inherit;
  border-radius: 0;
  background-color: transparent;
  opacity: 1;
}
::placeholder {
  opacity: 1;
}
@supports (not (-webkit-appearance: -apple-pay-button)) or (contain-intrinsic-size: 1px) {
  ::placeholder {
    color: color-mix(in oklab, currentcolor 50%, transparent);
  }
}
textarea {
  resize: vertical;
}
button,
input:where([type='button'], [type='reset'], [type='submit']),
::file-selector-button {
  appearance: button;
}
"#;

    const HIDDEN_ATTRIBUTE: &'static str = r#"
[hidden]:where(:not([hidden='until-found'])) {
  display: none !important;
}
"#;



    const PREFLIGHT_EXTRAS: &'static str = r#"
/* Minor typographic and element styles */
abbr:where([title]) { -webkit-text-decoration: underline dotted; text-decoration: underline dotted; }
b, strong { font-weight: bolder; }
code, kbd, samp, pre { font-family: --theme(--default-mono-font-family, ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace); font-size: 1em; }
small { font-size: 80%; }
sub, sup { font-size: 75%; line-height: 0; position: relative; vertical-align: baseline; }
sub { bottom: -0.25em; }
sup { top: -0.5em; }
progress { vertical-align: baseline; }
summary { display: list-item; }
"#;


    const PREFLIGHT_COMPATIBILITY_FIXES: &'static str = r#"
/* Browser-specific compatibility fixes */
:-moz-focusring { outline: auto; }
:-moz-ui-invalid { box-shadow: none; }
:where(select:is([multiple], [size])) optgroup { font-weight: bolder; }
:where(select:is([multiple], [size])) optgroup option { padding-inline-start: 20px; }
::file-selector-button { margin-inline-end: 4px; }
::-webkit-search-decoration { -webkit-appearance: none; }
::-webkit-date-and-time-value { min-height: 1lh; text-align: inherit; }
::-webkit-datetime-edit { display: inline-flex; }
::-webkit-datetime-edit-fields-wrapper, ::-webkit-datetime-edit-fields-wrapper, ::-webkit-datetime-edit, ::-webkit-datetime-edit-year-field, ::-webkit-datetime-edit-month-field, ::-webkit-datetime-edit-day-field, ::-webkit-datetime-edit-hour-field, ::-webkit-datetime-edit-minute-field, ::-webkit-datetime-edit-second-field, ::-webkit-datetime-edit-millisecond-field, ::-webkit-datetime-edit-meridiem-field { padding-block: 0; }
::-webkit-calendar-picker-indicator { line-height: 1; }
::-webkit-inner-spin-button, ::-webkit-outer-spin-button { height: auto; }
"#;



    // Creates a Preflight instance with the COMPLETE set of Tailwind v4 styles.
    pub fn full() -> Self {
        Self::default()
    }


    /// Creates a Preflight instance without the "extras" and "compatibility fixes" styles.
    pub fn core() -> Self {
        Self {
            specific_extras: false,
            compatibility_fixes: false,
            ..Self::default()
        }
    }

    /// Appends custom CSS to the end of the preflight styles.
    pub fn add_custom(&mut self, custom: &str) {
        if !self.custom.is_empty() {
            self.custom.push('\n');
        }
        self.custom.push_str(custom);
    }
  }


impl Display for PreflightSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.disable {
            return Ok(());
        }
        
        // Generate the built-in styles based on the boolean flags
        if self.global_reset {
            writeln!(f, "{}", Self::GLOBAL_RESET.trim())?;
        }
        if self.html_base {
            writeln!(f, "{}", Self::HTML_BASE.trim())?;
        }
        if self.unstyle_headings {
            writeln!(f, "{}", Self::UNSTYLE_HEADINGS.trim())?;
        }
        if self.unstyle_links {
            writeln!(f, "{}", Self::UNSTYLE_LINKS.trim())?;
        }
        if self.unstyle_lists {
            writeln!(f, "{}", Self::UNSTYLE_LISTS.trim())?;
        }
        if self.block_level_media {
            writeln!(f, "{}", Self::BLOCK_LEVEL_MEDIA.trim())?;
        }
        if self.reset_tables {
            writeln!(f, "{}", Self::RESET_TABLES.trim())?;
        }
        if self.reset_forms {
            writeln!(f, "{}", Self::RESET_FORMS.trim())?;
        }
        if self.hidden_attribute {
            writeln!(f, "{}", Self::HIDDEN_ATTRIBUTE.trim())?;
        }
        if self.specific_extras {
            writeln!(f, "{}", Self::PREFLIGHT_EXTRAS.trim())?;
        }
        if self.compatibility_fixes {
            writeln!(f, "{}", Self::PREFLIGHT_COMPATIBILITY_FIXES.trim())?;
        }

        // Append the user's custom styles at the very end
        if !self.custom.is_empty() {
            writeln!(f, "{}", self.custom.trim())?;
        }

        Ok(())
    }
}