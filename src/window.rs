use gpui::*;
use gpui_component::TitleBar;

pub fn get_window_options(cx: &mut App) -> WindowOptions {
    let mut window_size = size(px(1600.), px(1200.));
    if let Some(display) = cx.primary_display() {
        let display_size = display.bounds().size;
        window_size.width = display_size.width.min(display_size.width * 0.85);
        window_size.height = display_size.height.min(display_size.height * 0.85);
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
