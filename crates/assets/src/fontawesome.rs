extern crate gpui;
use gpui::{Font, FontStyle, SharedString, FontFeatures, FontWeight};

/// FontAwesome Pro font configurations
#[derive(Debug, Clone)]
pub struct FontAwesomeConfig {
    pub family: SharedString,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub unicode: &'static str,
}

impl FontAwesomeConfig {
    /// Classic Thin weight (100)
    pub fn thin() -> Self {
        Self {
            family: "Font Awesome 6 Pro Thin".into(),
            weight: FontWeight(100.0),
            style: FontStyle::Normal,
            unicode: "", // Will be set by icon
        }
    }

    /// Light weight (300)
    pub fn light() -> Self {
        Self {
            family: "Font Awesome 6 Pro Light".into(),
            weight: FontWeight::LIGHT,
            style: FontStyle::Normal,
            unicode: "",
        }
    }

    /// Regular weight (400)
    pub fn regular() -> Self {
        Self {
            family: "Font Awesome 6 Pro Regular".into(),
            weight: FontWeight::NORMAL,
            style: FontStyle::Normal,
            unicode: "",
        }
    }

    /// Solid weight (700)
    pub fn solid() -> Self {
        Self {
            family: "Font Awesome 6 Pro Solid".into(),
            weight: FontWeight::BOLD,
            style: FontStyle::Normal,
            unicode: "",
        }
    }

    /// Duotone weight (900)
    pub fn duotone() -> Self {
        Self {
            family: "Font Awesome 6 Pro Duotone".into(),
            weight: FontWeight::BLACK,
            style: FontStyle::Normal,
            unicode: "",
        }
    }
}

/// Font weight mappings for FontAwesome Pro
#[derive(Debug, Clone)]
pub enum FontAwesomeWeight {
    Thin,
    Light,
    Regular,
    Solid,
    Duotone,
}

impl FontAwesomeWeight {
    pub fn as_font_weight(&self) -> FontWeight {
        match self {
            FontAwesomeWeight::Thin => gpui::text_system::FontWeight::THIN,
            FontAwesomeWeight::Light => gpui::text_system::FontWeight::LIGHT,
            FontAwesomeWeight::Regular => gpui::text_system::FontWeight::NORMAL,
            FontAwesomeWeight::Solid => gpui::text_system::FontWeight::BOLD,
            FontAwesomeWeight::Duotone => gpui::text_system::FontWeight::BLACK,
        }
    }

    pub fn as_number(&self) -> u32 {
        match self {
            FontAwesomeWeight::Thin => 100,
            FontAwesomeWeight::Light => 300,
            FontAwesomeWeight::Regular => 400,
            FontAwesomeWeight::Solid => 700,
            FontAwesomeWeight::Duotone => 900,
        }
    }
}

/// FontAwesome icon configurations
#[derive(Debug, Clone)]
pub struct FontAwesomeIcon {
    pub unicode: &'static str,
    pub weight: FontAwesomeWeight,
    pub class: &'static str,
}

/// Common FontAwesome icons mapped to GPUT IconName
pub const FONTAWESOME_ICONS: &[(&str, FontAwesomeIcon)] = &[
    // Action Icons
    ("check", FontAwesomeIcon {
        unicode: "\u{f00c}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-check",
    }),
    ("xmark", FontAwesomeIcon {
        unicode: "\u{f00d}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-xmark",
    }),
    ("plus", FontAwesomeIcon {
        unicode: "\u{f067}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-plus",
    }),
    ("minus", FontAwesomeIcon {
        unicode: "\u{f068}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-minus",
    }),
    
    // Navigation Icons
    ("chevron-up", FontAwesomeIcon {
        unicode: "\u{f077}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-chevron-up",
    }),
    ("chevron-down", FontAwesomeIcon {
        unicode: "\u{f078}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-chevron-down",
    }),
    ("chevron-left", FontAwesomeIcon {
        unicode: "\u{f053}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-chevron-left",
    }),
    ("chevron-right", FontAwesomeIcon {
        unicode: "\u{f054}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-chevron-right",
    }),
    
    // UI Icons
    ("search", FontAwesomeIcon {
        unicode: "\u{f002}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-magnifying-glass",
    }),
    ("info", FontAwesomeIcon {
        unicode: "\u{f129}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-circle-info",
    }),
    ("settings", FontAwesomeIcon {
        unicode: "\u{f013}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-gear",
    }),
    ("close", FontAwesomeIcon {
        unicode: "\u{f00d}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-xmark",
    }),
    
    // Status Icons
    ("heart", FontAwesomeIcon {
        unicode: "\u{f004}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-heart",
    }),
    ("star", FontAwesomeIcon {
        unicode: "\u{f005}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-star",
    }),
    ("bell", FontAwesomeIcon {
        unicode: "\u{f0f3}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-bell",
    }),
    ("eye", FontAwesomeIcon {
        unicode: "\u{f06e}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-eye",
    }),
    
    // Content Icons
    ("calendar", FontAwesomeIcon {
        unicode: "\u{f133}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-calendar",
    }),
    ("file", FontAwesomeIcon {
        unicode: "\u{f15b}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-file",
    }),
    ("folder", FontAwesomeIcon {
        unicode: "\u{f07b}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-folder",
    }),
    ("user", FontAwesomeIcon {
        unicode: "\u{f007}",
        weight: FontAwesomeWeight::Solid,
        class: "fa-solid fa-user",
    }),
    
    // Classic Thin Icons (for special use cases)
    ("check-thin", FontAwesomeIcon {
        unicode: "\u{f00c}",
        weight: FontAwesomeWeight::Thin,
        class: "fa-thin fa-check",
    }),
    ("heart-thin", FontAwesomeIcon {
        unicode: "\u{f004}",
        weight: FontAwesomeWeight::Thin,
        class: "fa-thin fa-heart",
    }),
    ("star-thin", FontAwesomeIcon {
        unicode: "\u{f005}",
        weight: FontAwesomeWeight::Thin,
        class: "fa-thin fa-star",
    }),
];

/// Helper function to get FontAwesome icon by name
pub fn get_fontawesome_icon(name: &str) -> Option<FontAwesomeIcon> {
    FONTAWESOME_ICONS
        .iter()
        .find(|(icon_name, _)| *icon_name == name)
        .map(|(_, icon)| icon.clone())
}

/// FontAwesome font loading configuration
pub struct FontAwesomeLoader;

impl FontAwesomeLoader {
    /// Load FontAwesome fonts with specified weights
    pub fn load_fonts() -> Vec<Font> {
        vec![
            Font {
                family: FontAwesomeConfig::thin().family,
                features: Default::default(),
                fallbacks: None,
                weight: FontAwesomeConfig::thin().weight,
                style: FontAwesomeConfig::thin().style,
            },
            Font {
                family: FontAwesomeConfig::light().family,
                features: Default::default(),
                fallbacks: None,
                weight: FontAwesomeConfig::light().weight,
                style: FontAwesomeConfig::light().style,
            },
            Font {
                family: FontAwesomeConfig::regular().family,
                features: Default::default(),
                fallbacks: None,
                weight: FontAwesomeConfig::regular().weight,
                style: FontAwesomeConfig::regular().style,
            },
            Font {
                family: FontAwesomeConfig::solid().family,
                features: Default::default(),
                fallbacks: None,
                weight: FontAwesomeConfig::solid().weight,
                style: FontAwesomeConfig::solid().style,
            },
            Font {
                family: FontAwesomeConfig::duotone().family,
                features: Default::default(),
                fallbacks: None,
                weight: FontAwesomeConfig::duotone().weight,
                style: FontAwesomeConfig::duotone().style,
            },
        ]
    }
    
    /// Get CSS for FontAwesome integration
    pub fn get_css() -> &'static str {
        r#"
        @font-face {
            font-family: 'Font Awesome 6 Pro';
            src: url('/assets/fonts/fontawesome/fa-thin-100.woff2') format('woff2');
            font-weight: 100;
            font-style: normal;
            font-display: swap;
        }
        @font-face {
            font-family: 'Font Awesome 6 Pro';
            src: url('/assets/fonts/fontawesome/fa-light-300.woff2') format('woff2');
            font-weight: 300;
            font-style: normal;
            font-display: swap;
        }
        @font-face {
            font-family: 'Font Awesome 6 Pro';
            src: url('/assets/fonts/fontawesome/fa-regular-400.woff2') format('woff2');
            font-weight: 400;
            font-style: normal;
            font-display: swap;
        }
        @font-face {
            font-family: 'Font Awesome 6 Pro';
            src: url('/assets/fonts/fontawesome/fa-solid-700.woff2') format('woff2');
            font-weight: 700;
            font-style: normal;
            font-display: swap;
        }
        @font-face {
            font-family: 'Font Awesome 6 Pro';
            src: url('/assets/fonts/fontawesome/fa-duotone-900.woff2') format('woff2');
            font-weight: 900;
            font-style: normal;
            font-display: swap;
        }
        "#
    }
}