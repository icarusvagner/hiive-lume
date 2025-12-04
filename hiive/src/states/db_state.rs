use gpui::{App, Global};
use lib_core::model::ModelManager;

pub struct ConnectionState {
	pub mm: Option<ModelManager>,
}

impl Global for ConnectionState {}

impl ConnectionState {
	pub fn init(cx: &mut App) {
		let mm = None;
		let this = ConnectionState { mm };

		cx.set_global(this);
		cx.spawn(async move |cx| {
			if let Ok(model_manager) = ModelManager::new().await {
				let _ =
					cx.update_global::<ConnectionState, _>(|mm_state, _| {
						mm_state.mm = Some(model_manager);
					});
			}
		})
		.detach();
	}
}
