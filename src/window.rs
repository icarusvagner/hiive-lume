use gpui::*;
use gpui_component::TitleBar;

pub fn get_window_options(cx: &mut App) -> WindowOptions {
    let mut window_size = size(px(1600.), px(1200.));
    if let Some(display) = cx.primary_display() {
        let display_size = display.bounds().size;
        let horizontal_margin = display_size.width * 0.10;
        let vertical_margin = display_size.height * 0.10;
        window_size.width = display_size.width - horizontal_margin * 2.0;
        window_size.height = display_size.height - vertical_margin * 2.0;
    }

    let bounds = Bounds::centered(None, window_size, cx);
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        titlebar: Some(TitleBar::title_bar_options()),
        kind: WindowKind::Normal,
        window_decorations: Some(WindowDecorations::Client),
        ..Default::default()
    }
}
