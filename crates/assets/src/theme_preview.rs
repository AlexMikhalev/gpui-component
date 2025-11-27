use gpui_component_assets::{IconConfig, IconSource, FontAwesomeWeight};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Theme preview configuration for development
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemePreview {
    /// Available themes
    pub themes: Vec<String>,
    /// Current active theme
    pub current_theme: String,
    /// Icon configuration for preview
    pub icon_config: IconConfig,
}

impl Default for ThemePreview {
    fn default() -> Self {
        Self {
            themes: vec![
                "default".to_string(),
                "shoelace".to_string(), 
                "web-awesome".to_string(),
            ],
            current_theme: "default".to_string(),
            icon_config: IconConfig::default(),
        }
    }
}

/// Theme preview manager for development and testing
pub struct ThemePreviewManager {
    preview_config: ThemePreview,
}

impl ThemePreviewManager {
    pub fn new() -> Self {
        Self {
            preview_config: ThemePreview::default(),
        }
    }

    /// Get current theme
    pub fn current_theme(&self) -> &str {
        &self.preview_config.current_theme
    }

    /// Set current theme
    pub fn set_theme(&mut self, theme: &str) {
        if self.preview_config.themes.contains(&theme.to_string()) {
            self.preview_config.current_theme = theme.to_string();
        }
    }

    /// Get available themes
    pub fn available_themes(&self) -> &[String] {
        &self.preview_config.themes
    }

    /// Get icon configuration
    pub fn icon_config(&self) -> &IconConfig {
        &self.preview_config.icon_config
    }

    /// Update icon configuration
    pub fn update_icon_config(&mut self, config: IconConfig) {
        self.preview_config.icon_config = config;
    }

    /// Toggle between FontAwesome weights for testing
    pub fn toggle_fontawesome_weight(&mut self) {
        let current_weight = self.preview_config.icon_config.fontawesome_weight;
        let new_weight = match current_weight {
            Some(FontAwesomeWeight::Regular) => Some(FontAwesomeWeight::Thin),
            Some(FontAwesomeWeight::Thin) => Some(FontAwesomeWeight::Regular),
            _ => Some(FontAwesomeWeight::Regular),
        };
        
        if let Some(new_weight) = new_weight {
            self.preview_config.icon_config.fontawesome_weight = new_weight;
        }
    }

    /// Toggle between icon sources for testing
    pub fn toggle_icon_source(&mut self) {
        let current_source = self.preview_config.icon_config.source;
        let new_source = match current_source {
            IconSource::Lucide => IconSource::FontAwesome,
            IconSource::FontAwesome => IconSource::Dual,
            IconSource::Dual => IconSource::Lucide,
        };
        
        self.preview_config.icon_config.source = new_source;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_preview_default() {
        let preview = ThemePreviewManager::new();
        assert_eq!(preview.current_theme(), "default");
        assert_eq!(preview.available_themes().len(), 3);
    }

    #[test]
    fn test_set_valid_theme() {
        let mut preview = ThemePreviewManager::new();
        preview.set_theme("shoelace");
        assert_eq!(preview.current_theme(), "shoelace");
    }

    #[test]
    fn test_set_invalid_theme() {
        let mut preview = ThemePreviewManager::new();
        preview.set_theme("invalid-theme");
        assert_eq!(preview.current_theme(), "default"); // Should remain unchanged
    }

    #[test]
    fn test_toggle_fontawesome_weight() {
        let mut preview = ThemePreviewManager::new();
        preview.toggle_fontawesome_weight();
        assert_eq!(preview.icon_config().fontawesome_weight, Some(FontAwesomeWeight::Thin));
    }

    #[test]
    fn test_toggle_icon_source() {
        let mut preview = ThemePreviewManager::new();
        preview.toggle_icon_source();
        assert_eq!(preview.icon_config().source, IconSource::FontAwesome);
    }
}