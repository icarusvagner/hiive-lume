use gpui::*;
use gpui_component::{ActiveTheme, Root, VirtualListScrollHandle, v_virtual_list};
use std::rc::Rc;

pub struct ListViewExample {
    items: Vec<String>,
    item_sizes: Rc<Vec<Size<Pixels>>>,
    scroll_handle: VirtualListScrollHandle,
}

impl ListViewExample {
    fn new(_cx: &mut Context<Self>) -> Self {
        let items = (0..5000).map(|i| format!("Item {}", i)).collect::<Vec<_>>();
        let item_sizes = Rc::new(items.iter().map(|_| size(px(200.), px(30.))).collect());

        Self {
            items,
            item_sizes,
            scroll_handle: VirtualListScrollHandle::new(),
        }
    }
}

impl Render for ListViewExample {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_virtual_list(
            cx.entity().clone(),
            "my-list",
            self.item_sizes.clone(),
            |_view, visible_range, _, cx| {
                visible_range
                    .map(|ix| {
                        div()
                            .h(px(30.))
                            .w_full()
                            .bg(cx.theme().secondary)
                            .child(format!("Item {}", ix))
                    })
                    .collect()
            },
        )
        .track_scroll(&self.scroll_handle)
    }
}

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        cx.activate(true);

        cx.on_window_closed(|cx| {
            if cx.windows().len() == 0 {
                cx.quit();
            }
        })
        .detach();

        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(500.), px(450.)), cx);
        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    kind: WindowKind::Normal,
                    window_decorations: Some(WindowDecorations::Client),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| ListViewExample::new(cx));

                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
