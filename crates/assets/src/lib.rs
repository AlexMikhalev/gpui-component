use anyhow::anyhow;
use gpui::{AssetSource, Result, SharedString};
use rust_embed::RustEmbed;
use std::borrow::Cow;

pub mod fontawesome;
pub mod icon_mapping;

/// Embed application assets for GPUI Component.
///
/// This assets provides:
/// - Icons svg files for [IconName](https://docs.rs/gpui-component/latest/gpui_component/enum.IconName.html)
/// - FontAwesome Pro integration with multiple font weights
/// - Dual icon system supporting both Lucide and FontAwesome icons
///
/// ```
/// use gpui::*;
/// use gpui_component_assets::{Assets, fontawesome::FontAwesomeLoader};
///
/// let app = Application::new().with_assets(Assets);
/// ```
#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "icons/**/*.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("could not find asset at path \"{path}\""))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

/// Initialize FontAwesome integration
pub fn init_fontawesome() {
    // Load FontAwesome fonts
    let _fonts = fontawesome::FontAwesomeLoader::load_fonts();
    
    // FontAwesome fonts would be loaded here in a real implementation
    // This is a placeholder for the actual font loading logic
}

/// Get FontAwesome CSS for web integration
pub fn get_fontawesome_css() -> &'static str {
    fontawesome::FontAwesomeLoader::get_css()
}
