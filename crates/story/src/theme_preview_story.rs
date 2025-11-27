use gpui::*;
use gpui_component::prelude::*;
use gpui_component_assets::{IconConfig, IconSource, FontAwesomeWeight, ThemePreviewManager};

struct ThemePreviewStory {
    preview_manager: State<ThemePreviewManager>,
}

impl ThemePreviewStory {
    fn new() -> Self {
        Self {
            preview_manager: State::new(ThemePreviewManager::new()),
        }
    }
}

impl Render for ThemePreviewStory {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(cx.theme().background)
            .p_8()
            .child(
                div()
                    .mb_6()
                    .child(
                        div()
                            .text_lg()
                            .font_semibold()
                            .child("Theme Preview System")
                    )
                    .child(
                        div()
                            .mb_4()
                            .text_sm()
                            .font_medium()
                            .child("Current theme: ")
                            .child(
                                span()
                                    .font_family("monospace")
                                    .text_xs()
                                    .child(self.preview_manager.read(cx).current_theme())
                            )
                    )
                    .child(
                        div()
                            .mb_4()
                            .text_sm()
                            .font_medium()
                            .child("Available themes:")
                    )
                    .for_each(|theme| {
                        self.preview_manager.read(cx).available_themes().iter().map(|theme| {
                            div()
                                .mt_2()
                                .p_2()
                                .bg(if self.preview_manager.read(cx).current_theme() == theme { 
                                    cx.theme().primary 
                                } else { 
                                    cx.theme().muted_background 
                                })
                                .rounded_md()
                                .px_2()
                                .py_1()
                                .cursor_pointer()
                                .on_click(move |_, window, cx| {
                                    self.preview_manager.write(cx).set_theme(theme);
                                })
                                .child(
                                    div()
                                        .inline_flex()
                                        .items_center()
                                        .gap_2()
                                        .p_2()
                                        .bg(if theme == "shoelace" { 
                                            gpui::blue() 
                                        } else if theme == "web-awesome" { 
                                            gpui::indigo() 
                                        } else { 
                                            gpui::gray() 
                                        })
                                        .rounded_full()
                                        .border_2()
                                        .border_color(if theme == "shoelace" { 
                                            gpui::blue() 
                                        } else if theme == "web-awesome" { 
                                            gpui::indigo() 
                                        } else { 
                                            gpui::gray() 
                                        })
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .p_1()
                                        .text_color(if theme == "shoelace" { 
                                            gpui::white() 
                                        } else if theme == "web-awesome" { 
                                            gpui::white() 
                                        } else { 
                                            gpui::foreground() 
                                        })
                                        .text_xs()
                                        .font_medium()
                                        .child(theme)
                                )
                        })
                    })
                    .child(
                        div()
                            .mt_6()
                            .mb_4()
                            .child(
                                div()
                                    .text_sm()
                                    .font_medium()
                                    .child("Icon Configuration:")
                            )
                            .child(
                                div()
                                    .mt_2()
                                    .flex()
                                    .gap_4()
                                    .child(
                                        Button::new("weight-thin")
                                            .label("Thin (100)")
                                            .small()
                                            .icon_config(IconConfig {
                                                source: IconSource::FontAwesome,
                                                fontawesome_weight: Some(FontAwesomeWeight::Thin),
                                                fallback_to_lucide: true,
                                            })
                                            .on_click(move |_, window, cx| {
                                                self.preview_manager.write(cx).update_icon_config(IconConfig {
                                                    source: IconSource::FontAwesome,
                                                    fontawesome_weight: Some(FontAwesomeWeight::Thin),
                                                    fallback_to_lucide: true,
                                                });
                                            })
                                    )
                                    .child(
                                        Button::new("weight-regular")
                                            .label("Regular (400)")
                                            .small()
                                            .icon_config(IconConfig {
                                                source: IconSource::FontAwesome,
                                                fontawesome_weight: Some(FontAwesomeWeight::Regular),
                                                fallback_to_lucide: true,
                                            })
                                            .on_click(move |_, window, cx| {
                                                self.preview_manager.write(cx).update_icon_config(IconConfig {
                                                    source: IconSource::FontAwesome,
                                                    fontawesome_weight: Some(FontAwesomeWeight::Regular),
                                                    fallback_to_lucide: true,
                                                });
                                            })
                                    )
                                    .child(
                                        Button::new("source-lucide")
                                            .label("Lucide Only")
                                            .small()
                                            .icon_config(IconConfig {
                                                source: IconSource::Lucide,
                                                fontawesome_weight: None,
                                                fallback_to_lucide: false,
                                            })
                                            .on_click(move |_, window, cx| {
                                                self.preview_manager.write(cx).update_icon_config(IconConfig {
                                                    source: IconSource::Lucide,
                                                    fontawesome_weight: None,
                                                    fallback_to_lucide: false,
                                                });
                                            })
                                    )
                                    .child(
                                        Button::new("source-fontawesome")
                                            .label("FontAwesome")
                                            .small()
                                            .icon_config(IconConfig {
                                                source: IconSource::FontAwesome,
                                                fontawesome_weight: Some(FontAwesomeWeight::Regular),
                                                fallback_to_lucide: true,
                                            })
                                            .on_click(move |_, window, cx| {
                                                self.preview_manager.write(cx).update_icon_config(IconConfig {
                                                    source: IconSource::FontAwesome,
                                                    fontawesome_weight: Some(FontAwesomeWeight::Regular),
                                                    fallback_to_lucide: true,
                                                });
                                            })
                                    )
                                    .child(
                                        Button::new("source-dual")
                                            .label("Dual System")
                                            .small()
                                            .icon_config(IconConfig {
                                                source: IconSource::Dual,
                                                fontawesome_weight: Some(FontAwesomeWeight::Regular),
                                                fallback_to_lucide: true,
                                            })
                                            .on_click(move |_, window, cx| {
                                                self.preview_manager.write(cx).update_icon_config(IconConfig {
                                                    source: IconSource::Dual,
                                                    fontawesome_weight: Some(FontAwesomeWeight::Regular),
                                                    fallback_to_lucide: true,
                                                });
                                            })
                                    )
                            )
                    )
                    .child(
                        div()
                            .mt_6()
                            .mb_4()
                            .child(
                                div()
                                    .text_sm()
                                    .font_medium()
                                    .child("Icons demonstrate theme-aware rendering:")
                            )
                            .child(
                                div()
                                    .mt_2()
                                    .flex()
                                    .gap_4()
                                    .child(
                                        Icon::new("check").icon_config(
                                            self.preview_manager.read(cx).icon_config().clone()
                                        )
                                    .child(
                                        Icon::new("close").icon_config(
                                            self.preview_manager.read(cx).icon_config().clone()
                                        )
                                    .child(
                                        Icon::new("plus").icon_config(
                                            self.preview_manager.read(cx).icon_config().clone()
                                        )
                                    .child(
                                        Icon::new("heart").icon_config(
                                            self.preview_manager.read(cx).icon_config().clone()
                                        )
                                    .child(
                                        Icon::new("star").icon_config(
                                            self.preview_manager.read(cx).icon_config().clone()
                                        )
                                    )
                            )
                    )
            )
    }
}