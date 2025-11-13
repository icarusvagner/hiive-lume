use std::time::Duration;

use gpui::*;
use gpui_component::{
    ActiveTheme, IconName, StyledExt,
    avatar::Avatar,
    button::{Button, ButtonVariants},
    label::Label,
    menu::DropdownMenu,
};

use crate::{
    data::home::header_menu,
    states::home_layout::{HomeActiveLayout, HomeLayout},
    workspace::global_actions::{LogoutAction, ProfileAction, SettingsAction},
};

pub struct HomeHeader {}

impl HomeHeader {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn navigate_home_content(&self, layout: HomeActiveLayout, cx: &mut App) {
        let _ = cx.update_global::<HomeLayout, _>(|state, _| {
            if !state.home.eq(&layout) {
                state.home = HomeActiveLayout::Loading;
            }
        });

        cx.spawn(async move |cx| {
            cx.background_executor()
                .timer(Duration::from_millis(500))
                .await;

            let _ = cx.update_global::<HomeLayout, _>(|state, _| {
                state.home = layout;
            });
        })
        .detach();
    }

    fn menu_buttons(&self, cx: &mut Context<Self>) -> Vec<Button> {
        let mut buttons = Vec::new();

        for (indx, btn) in header_menu::HeaderMenu::all_data().iter().enumerate() {
            let layout = btn.goto_layout;

            let button = Button::new(indx)
                .label(btn.label.clone())
                .icon(btn.icon.clone())
                .ghost()
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.navigate_home_content(layout, cx);
                }));

            buttons.push(button);
        }

        buttons
    }

    fn menu_profile(&self, cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .items_center()
            .gap_2()
            .py(px(2.5))
            .px(px(5.0))
            .rounded_full()
            .bg(cx.theme().secondary)
            .child(
                Avatar::new()
                    .size(px(35.))
                    .placeholder(IconName::CircleUser)
                    .bg(cx.theme().primary),
            )
            .child(
                Button::new("header-menu-btn")
                    .cursor_pointer()
                    .label("John Doe")
                    .dropdown_menu(|menu, _window, _cx| {
                        menu.menu("Profile", Box::new(ProfileAction))
                            .menu("Settings", Box::new(SettingsAction))
                            .separator()
                            .menu("Logout", Box::new(LogoutAction))
                    }),
            )
    }
}

impl Render for HomeHeader {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().accent)
            .py(px(4.5))
            .px(px(5.0))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .items_center()
                            .child(img("images/hiive-logo.png").size(px(30.)))
                            .child(Label::new("Hiive Lume").font_bold().text_lg()),
                    )
                    .child(div().w(px(20.)))
                    .child(div().flex().gap_2().children(self.menu_buttons(cx))),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("btn-search")
                            .icon(IconName::Search)
                            .rounded_full()
                            .ghost(),
                    )
                    .child(
                        Button::new("btn-notif")
                            .icon(IconName::Bell)
                            .rounded_full()
                            .ghost(),
                    )
                    .child(self.menu_profile(cx)),
            )
    }
}
