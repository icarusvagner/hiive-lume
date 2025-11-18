use std::{collections::HashMap, rc::Rc, sync::LazyLock};

use gpui::*;
use gpui_component::{Theme, ThemeConfig, ThemeMode, ThemeSet};

pub static THEMES: LazyLock<HashMap<SharedString, ThemeConfig>> = LazyLock::new(|| {
    fn parse_theme(source: &str) -> ThemeSet {
        serde_json::from_str(source).unwrap()
    }

    let mut themes = HashMap::new();

    for source in [
        include_str!("../../assets/themes/catppuccin.json"),
        include_str!("../../assets/themes/tokyonight.json"),
        include_str!("../../assets/themes/everforest.json"),
        include_str!("../../assets/themes/jellybeans.json"),
        include_str!("../../assets/themes/adventure.json"),
        include_str!("../../assets/themes/ayu.json"),
        include_str!("../../assets/themes/macos-classic.json"),
    ] {
        let theme_set = parse_theme(source);
        for theme in theme_set.themes {
            themes.insert(theme.name.clone(), theme);
        }
    }

    themes
});

pub fn change_color_mode(mode: ThemeMode, _window: &mut Window, cx: &mut App) {
    let theme_name = match mode {
        ThemeMode::Light => "macOS Classic Light",
        ThemeMode::Dark => "macOS Classic Dark",
    };

    if let Some(theme_config) = THEMES.get(theme_name) {
        let theme_config = Rc::new(theme_config.clone());
        let theme = Theme::global_mut(cx);
        theme.mode = theme_config.mode;
        theme.apply_config(&theme_config);
    }
}
