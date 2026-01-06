use gpui::*;
use gpui_component::{
	scroll::ScrollableElement, select::{Select, SelectState}, *
};

pub struct HelloWorld;

impl Render for HelloWorld {
	fn render(
		&mut self,
		window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		let fruits = vec!["Apple", "Orange", "Banana", "Grape", "Pineapple"];

		let select_state = cx.new(|cx| {
			SelectState::new(fruits, Some(IndexPath::default()), window, cx)
		});

		select_state.update(cx, |state, cx| {
			state.set_selected_index(
				Some(IndexPath::default().row(1)),
				window,
				cx,
			);
			cx.notify();
		});

		v_flex()
			.gap_2()
			.p_4()
			.child("Scrollable Content")
			.child(Select::new(&select_state))
			.children((0..100).map(|i| {
				div()
					.h(px(40.))
					.w_full()
					.bg(cx.theme().secondary)
					.child(format!("Item {}", i))
			}))
			.overflow_y_scrollbar()
			.children(Root::render_notification_layer(window, cx))
			.children(Root::render_sheet_layer(window, cx))
			.children(Root::render_dialog_layer(window, cx))
	}
}

fn main() {
	let app = Application::new();

	app.run(move |cx| {
		// This must be called before using any GPUI Component features.
		gpui_component::init(cx);
		let bounds = Bounds::centered(None, size(px(550.), px(500.)), cx);

		cx.spawn(async move |cx| {
			cx.open_window(
				WindowOptions {
					window_bounds: Some(WindowBounds::Windowed(bounds)),
					kind: WindowKind::Normal,
					window_decorations: Some(WindowDecorations::Client),
					..Default::default()
				},
				|window, cx| {
					let view = cx.new(|_| HelloWorld);
					// This first level on the window, should be a Root.
					cx.new(|cx| Root::new(view, window, cx))
				},
			)?;

			Ok::<_, anyhow::Error>(())
		})
		.detach();
	});
}
