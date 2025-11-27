/// Icon mapping between GPUT IconName and FontAwesome
use crate::fontawesome::{FontAwesomeIcon, FontAwesomeWeight, get_fontawesome_icon};

/// Source for icons
#[derive(Debug, Clone)]
pub enum IconSource {
    /// Use GPUT's built-in Lucide icons
    Lucide,
    /// Use FontAwesome icons
    FontAwesome,
    /// Use dual system with FontAwesome as primary
    Dual,
}

/// Icon selection configuration
#[derive(Debug, Clone)]
pub struct IconConfig {
    pub source: IconSource,
    pub fontawesome_weight: Option<FontAwesomeWeight>,
    pub fallback_to_lucide: bool,
}

impl Default for IconConfig {
    fn default() -> Self {
        Self {
            source: IconSource::Dual,
            fontawesome_weight: Some(FontAwesomeWeight::Regular),
            fallback_to_lucide: true,
        }
    }
}

/// Mapping from GPUT IconName to FontAwesome icons
pub struct IconMapping;

impl IconMapping {
    /// Get FontAwesome equivalent for GPUT IconName
    pub fn get_fontawesome_equivalent(icon_name: &str, weight: &FontAwesomeWeight) -> Option<FontAwesomeIcon> {
        let mapping = match icon_name {
            // Core action icons
            "check" => match weight {
                FontAwesomeWeight::Thin => get_fontawesome_icon("check-thin"),
                _ => get_fontawesome_icon("check"),
            },
            "close" => get_fontawesome_icon("xmark"),
            "plus" => get_fontawesome_icon("plus"),
            "minus" => get_fontawesome_icon("minus"),
            
            // Navigation
            "chevron-up" => get_fontawesome_icon("chevron-up"),
            "chevron-down" => get_fontawesome_icon("chevron-down"),
            "chevron-left" => get_fontawesome_icon("chevron-left"),
            "chevron-right" => get_fontawesome_icon("chevron-right"),
            "arrow-up" => get_fontawesome_icon("chevron-up"),
            "arrow-down" => get_fontawesome_icon("chevron-down"),
            "arrow-left" => get_fontawesome_icon("chevron-left"),
            "arrow-right" => get_fontawesome_icon("chevron-right"),
            
            // UI elements
            "search" => get_fontawesome_icon("search"),
            "settings" => get_fontawesome_icon("settings"),
            "settings-2" => get_fontawesome_icon("settings"),
            "menu" => get_fontawesome_icon("bars-3"), // Would need to add this
            "maximize" => get_fontawesome_icon("expand"),
            "minimize" => get_fontawesome_icon("compress"),
            
            // Status and feedback
            "info" => get_fontawesome_icon("info"),
            "warning" => get_fontawesome_icon("triangle-exclamation"),
            "error" => get_fontawesome_icon("circle-xmark"),
            "success" => get_fontawesome_icon("circle-check"),
            "alert" => get_fontawesome_icon("triangle-exclamation"),
            
            // Content and objects
            "calendar" => get_fontawesome_icon("calendar"),
            "file" => get_fontawesome_icon("file"),
            "folder" => get_fontawesome_icon("folder"),
            "folder-open" => get_fontawesome_icon("folder-open"),
            "user" => get_fontawesome_icon("user"),
            "users" => get_fontawesome_icon("users"),
            "heart" => match weight {
                FontAwesomeWeight::Thin => get_fontawesome_icon("heart-thin"),
                _ => get_fontawesome_icon("heart"),
            },
            "heart-off" => get_fontawesome_icon("heart-crack"),
            "star" => match weight {
                FontAwesomeWeight::Thin => get_fontawesome_icon("star-thin"),
                _ => get_fontawesome_icon("star"),
            },
            "star-off" => get_fontawesome_icon("star-half-stroke"),
            "bell" => get_fontawesome_icon("bell"),
            "eye" => get_fontawesome_icon("eye"),
            "eye-off" => get_fontawesome_icon("eye-slash"),
            
            // Actions
            "copy" => get_fontawesome_icon("copy"),
            "cut" => get_fontawesome_icon("scissors"),
            "paste" => get_fontawesome_icon("paste"),
            "delete" => get_fontawesome_icon("trash"),
            "edit" => get_fontawesome_icon("pencil"),
            "save" => get_fontawesome_icon("floppy-disk"),
            "load" => get_fontawesome_icon("download"),
            "upload" => get_fontawesome_icon("upload"),
            
            // Communication
            "mail" => get_fontawesome_icon("envelope"),
            "chat" => get_fontawesome_icon("message"),
            "phone" => get_fontawesome_icon("phone"),
            "video" => get_fontawesome_icon("video"),
            
            // Media
            "play" => get_fontawesome_icon("play"),
            "pause" => get_fontawesome_icon("pause"),
            "stop" => get_fontawesome_icon("stop"),
            "volume" => get_fontawesome_icon("volume-high"),
            "volume-off" => get_fontawesome_icon("volume-xmark"),
            
            // Technology
            "computer" => get_fontawesome_icon("computer"),
            "mobile" => get_fontawesome_icon("mobile"),
            "tablet" => get_fontawesome_icon("tablet"),
            "wifi" => get_fontawesome_icon("wifi"),
            "bluetooth" => get_fontawesome_icon("bluetooth"),
            
            // Weather and nature
            "sun" => get_fontawesome_icon("sun"),
            "moon" => get_fontawesome_icon("moon"),
            "cloud" => get_fontawesome_icon("cloud"),
            "rain" => get_fontawesome_icon("cloud-rain"),
            "snow" => get_fontawesome_icon("snowflake"),
            "wind" => get_fontawesome_icon("wind"),
            
            // Food and drinks
            "coffee" => get_fontawesome_icon("mug-hot"),
            "food" => get_fontawesome_icon("utensils"),
            "drink" => get_fontawesome_icon("wine-glass"),
            
            // Transport
            "car" => get_fontawesome_icon("car"),
            "bike" => get_fontawesome_icon("bicycle"),
            "bus" => get_fontawesome_icon("bus"),
            "train" => get_fontawesome_icon("train"),
            "plane" => get_fontawesome_icon("plane"),
            
            // Business and finance
            "money" => get_fontawesome_icon("dollar-sign"),
            "shopping" => get_fontawesome_icon("bag-shopping"),
            "cart" => get_fontawesome_icon("cart-shopping"),
            "credit-card" => get_fontawesome_icon("credit-card"),
            
            // Health and fitness
            "heartbeat" => get_fontawesome_icon("heart-pulse"),
            "pulse" => get_fontawesome_icon("heart-pulse"),
            "medicine" => get_fontawesome_icon("pills"),
            "hospital" => get_fontawesome_icon("hospital"),
            
            // Sports and games
            "ball" => get_fontawesome_icon("futbol"),
            "game" => get_fontawesome_icon("gamepad"),
            "music" => get_fontawesome_icon("music"),
            
            // Placeholder for icons not yet mapped
            _ => {
                // Return a generic icon or None
                None
            }
        };
        
        mapping
    }
    
    /// Check if an icon has FontAwesome equivalent
    pub fn has_fontawesome_equivalent(icon_name: &str) -> bool {
        Self::get_fontawesome_equivalent(icon_name, &FontAwesomeWeight::Regular).is_some()
    }
    
    /// Get all available FontAwesome weights for an icon
    pub fn get_available_weights(icon_name: &str) -> Vec<FontAwesomeWeight> {
        vec![
            FontAwesomeWeight::Thin,
            FontAwesomeWeight::Light,
            FontAwesomeWeight::Regular,
            FontAwesomeWeight::Solid,
            FontAwesomeWeight::Duotone,
        ]
            .into_iter()
            .filter(|weight| Self::has_fontawesome_equivalent_for_weight(icon_name, weight))
            .collect()
    }

    /// Check if icon has FontAwesome equivalent for specific weight
    fn has_fontawesome_equivalent_for_weight(icon_name: &str, weight: FontAwesomeWeight) -> bool {
        Self::get_fontawesome_equivalent(icon_name, &weight).is_some()
    }
    
    /// Convert GPUT icon size to FontAwesome equivalent
    pub fn convert_size(gput_size: f32, fontawesome_base_size: f32) -> f32 {
        // FontAwesome icons are typically 1em = 16px by default
        // GPUT uses various sizes, so we need to map them
        if gput_size <= 12.0 {
            0.75 // 12px -> 0.75em
        } else if gput_size <= 14.0 {
            0.875 // 14px -> 0.875em
        } else if gput_size <= 16.0 {
            1.0 // 16px -> 1em
        } else if gput_size <= 18.0 {
            1.125 // 18px -> 1.125em
        } else if gput_size <= 20.0 {
            1.25 // 20px -> 1.25em
        } else if gput_size <= 24.0 {
            1.5 // 24px -> 1.5em
        } else {
            (gput_size / 16.0) as f32 // Default scaling
        }
    }
}

/// Icon rendering configuration for dual system
#[derive(Debug, Clone)]
pub struct DualIconConfig {
    pub primary_source: IconSource,
    pub fallback_enabled: bool,
    pub weight_preference: FontAwesomeWeight,
    pub size_conversion: bool,
}

impl Default for DualIconConfig {
    fn default() -> Self {
        Self {
            primary_source: IconSource::FontAwesome,
            fallback_enabled: true,
            weight_preference: FontAwesomeWeight::Regular,
            size_conversion: true,
        }
    }
}

/// Utility functions for icon system
pub struct IconUtils;

impl IconUtils {
    /// Determine the best icon source for a given context
    pub fn select_best_source(
        icon_name: &str,
        config: &IconConfig,
    ) -> IconSource {
        match config.source {
            IconSource::Lucide => IconSource::Lucide,
            IconSource::FontAwesome => {
                if IconMapping::has_fontawesome_equivalent(icon_name) {
                    IconSource::FontAwesome
                } else if config.fallback_to_lucide {
                    IconSource::Lucide
                } else {
                    IconSource::FontAwesome // Force FontAwesome even if not available
                }
            }
            IconSource::Dual => {
                if IconMapping::has_fontawesome_equivalent(icon_name) {
                    IconSource::FontAwesome
                } else if config.fallback_to_lucide {
                    IconSource::Lucide
                } else {
                    IconSource::FontAwesome
                }
            }
        }
    }
    
    /// Generate CSS classes for FontAwesome icons
    pub fn generate_fontawesome_classes(icon: &FontAwesomeIcon) -> String {
        format!("{} {}", "fa-solid", icon.class)
    }
    
    /// Generate inline CSS for FontAwesome icon
    pub fn generate_fontawesome_css(icon: &FontAwesomeIcon) -> String {
        let weight_css = match icon.weight {
            FontAwesomeWeight::Thin => "font-weight: 100;",
            FontAwesomeWeight::Light => "font-weight: 300;",
            FontAwesomeWeight::Regular => "font-weight: 400;",
            FontAwesomeWeight::Solid => "font-weight: 700;",
            FontAwesomeWeight::Duotone => "font-weight: 900;",
        };
        
        format!(
            "{}\nfont-family: 'Font Awesome 6 Pro';",
            weight_css
        )
    }
}