use gpui::*;
use gpui_component::{
	Icon, IconName, Sizable, WindowExt, button::{Button, ButtonVariants}, form::{field, v_form}, input::{Input, InputState}, notification::NotificationType
};

use crate::{
	core::handlers::handlers_login::{LoginPayload, handlers_login}, states::{
		auth_state::AuthState, db_state::ConnectionState, show_layout::LayoutState
	}
};

pub struct LoginForm {
	username: Entity<InputState>,
	password: Entity<InputState>,
	loading: bool,
}

impl LoginForm {
	pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
		cx.new(|cx| {
			let username = cx.new(|cx| {
				InputState::new(window, cx).placeholder("Enter admin username")
			});
			let password = cx.new(|cx| {
				InputState::new(window, cx)
					.placeholder("Enter admin password")
					.masked(true)
			});

			Self { username, password, loading: false }
		})
	}

	pub fn _clear(&mut self, window: &mut Window, cx: &mut Context<Self>) {
		let _ =
			self.username.update(cx, |this, cx| this.set_value("", window, cx));
		let _ =
			self.password.update(cx, |this, cx| this.set_value("", window, cx));

		cx.notify();
	}

	fn _validate_empty_input(&mut self, cx: &mut Context<Self>) -> bool {
		!self.username.read(cx).value().is_empty()
			&& !self.password.read(cx).value().is_empty()
	}

	fn auth_login(&mut self, window: &mut Window, cx: &mut Context<Self>) {
		if self.loading {
			return;
		}

		if let Some(mm_state) = cx.global::<ConnectionState>().mm.clone() {
			self.loading = true;
			cx.notify();

			if self._validate_empty_input(cx) {
				let payload = LoginPayload {
					username: self.username.read(cx).value().to_string(),
					password: self.password.read(cx).value().to_string(),
				};
				let entity = cx.entity();

				cx.spawn_in(window, async move |_, cx| {
					let result = handlers_login(&mm_state, payload).await;

					let _ = cx.update(|window, cx| match result {
						Ok(_) => {
							cx.update_global::<LayoutState, _>(
								move |state, _| {
									state.layout = crate::states::show_layout::ActiveLayout::Home;
								},
							);
						}
						Err(err) => {
							let err_msg: SharedString =
								format!("{}", err.to_string()).into();
							tracing::error!(
								"{:<12} - {}",
								"SOMETHING WENT WRONG",
								err.to_string()
							);
							window.push_notification(
								(NotificationType::Error, err_msg),
								cx,
							);
						}
					});

					cx.update_entity(&entity, |form, cx| {
						form.loading = false;
						cx.notify();
					})
					.ok();
				})
				.detach();
			} else {
				window.push_notification(
					(NotificationType::Error, "Input fields are required"),
					cx,
				);
				self.loading = false;
				cx.notify();
			}
		}
	}
}

impl Render for LoginForm {
	fn render(
		&mut self,
		_window: &mut Window,
		cx: &mut Context<Self>,
	) -> impl IntoElement {
		v_form()
			.large()
			.gap(px(12.))
			.mb_4()
			// Username
			.child(
				field().col_span(2).label("Username").required(true).child(
					Input::new(&self.username)
						.prefix(Icon::new(IconName::User))
						.large(),
				),
			)
			.mb_4()
			// Password
			.child(
				field().col_span(2).label("Password").required(true).child(
					Input::new(&self.password)
						.prefix(Icon::new(
							Icon::empty().path("icons/custom/lock-outline.svg"),
						))
						.mask_toggle()
						.large(),
				),
			)
			.mb_8()
			// Submit button
			.child(
				field().col_span(2).label_indent(false).child(
					Button::new("submit-btn")
						.mt_4()
						.label("Login")
						.primary()
						.cursor_pointer()
						.w_full()
						.large()
						.py_5()
						.loading(self.loading)
						.on_click(cx.listener(|this, _, win, cx| {
							this.auth_login(win, cx)
						})),
				),
			)
	}
}
