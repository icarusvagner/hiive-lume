use chrono::{Local, TimeZone};
use gpui::*;
use gpui_component::{
    ActiveTheme, Sizable, StyledExt, WindowExt,
    button::{Button, ButtonVariants},
    date_picker::{DatePicker, DatePickerState},
    form::{field, v_form},
    h_flex,
    input::{Input, InputState},
    label::Label,
    notification::NotificationType,
    v_flex,
};

pub struct CreateEmployee {
    // Personal Info
    firstname: Entity<InputState>,
    middlename: Entity<InputState>,
    lastname: Entity<InputState>,
    date_of_birth: Entity<DatePickerState>,
    suffix: Entity<InputState>,

    // Contact Info
    email: Entity<InputState>,
    phone_number: Entity<InputState>,
    emergency_contact_name: Entity<InputState>,
    emergency_contact_number: Entity<InputState>,
}

impl CreateEmployee {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let firstname = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Enter employee firstname")
                    .clean_on_escape()
            });
            let middlename = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Enter employee middlename")
                    .clean_on_escape()
            });
            let lastname = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Enter employee lastname")
                    .clean_on_escape()
            });
            let date_of_birth = cx.new(|cx| {
                let mut picker = DatePickerState::new(window, cx);
                let early_2000 = Local
                    .with_ymd_and_hms(2000, 1, 1, 0, 0, 0)
                    .unwrap()
                    .naive_local()
                    .date();
                picker.set_date(early_2000, window, cx);

                picker
            });
            let suffix = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Enter suffix name")
                    .clean_on_escape()
            });
            let email = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Enter contact email")
                    .clean_on_escape()
            });
            let phone_number = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("(+63)987 1234 567")
                    .clean_on_escape()
            });
            let emergency_contact_name = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Enter emergency contact number")
                    .clean_on_escape()
            });
            let emergency_contact_number = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Enter emergency contact name")
                    .clean_on_escape()
            });

            CreateEmployee {
                firstname,
                middlename,
                lastname,
                date_of_birth,
                suffix,
                email,
                phone_number,
                emergency_contact_name,
                emergency_contact_number,
            }
        })
    }

    fn clear_input(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let early_2000 = Local
            .with_ymd_and_hms(2000, 1, 1, 0, 0, 0)
            .unwrap()
            .naive_local()
            .date();

        let _ = self
            .firstname
            .update(cx, |this, cx| this.set_value("", window, cx));
        let _ = self
            .middlename
            .update(cx, |this, cx| this.set_value("", window, cx));
        let _ = self
            .lastname
            .update(cx, |this, cx| this.set_value("", window, cx));
        let _ = self
            .date_of_birth
            .update(cx, |this, cx| this.set_date(early_2000, window, cx));
        let _ = self
            .suffix
            .update(cx, |this, cx| this.set_value("", window, cx));
        let _ = self
            .email
            .update(cx, |this, cx| this.set_value("", window, cx));
        let _ = self
            .phone_number
            .update(cx, |this, cx| this.set_value("", window, cx));
        let _ = self
            .emergency_contact_name
            .update(cx, |this, cx| this.set_value("", window, cx));
        let _ = self
            .emergency_contact_number
            .update(cx, |this, cx| this.set_value("", window, cx));

        window.push_notification(
            (NotificationType::Success, "Creat employee form cleared"),
            cx,
        );
        cx.notify();
    }

    fn render_header(&self, cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .px_10()
            .py_6()
            .bg(cx.theme().accent)
            .justify_between()
            .items_center()
            .child(
                div().flex().flex_col().gap_1().child(
                    div()
                        .child("Register Employee")
                        .text_size(AbsoluteLength::Pixels(px(22.0)))
                        .text_color(cx.theme().accent_foreground)
                        .font_thin(),
                ),
            )
    }

    fn render_form(&self, _window: &mut Window, cx: &mut Context<Self>) -> Div {
        div()
            .px_10()
            .py_6()
            .grid()
            .grid_cols(2)
            .gap_5()
            .child(
                v_flex()
                    .gap_8()
                    .child(
                        div()
                            .flex_grow()
                            .flex()
                            .w_full()
                            .bg(cx.theme().accent.opacity(0.60))
                            .rounded_xl()
                            .overflow_hidden(),
                    )
                    .child(
                        v_flex().gap_2().child(
                            Button::new("upload-image-btn")
                                .label("Upload Image")
                                .w_full()
                                .large()
                                .cursor_pointer()
                                .primary(),
                        ),
                    ),
            )
            .child(
                v_flex()
                    .gap_5()
                    .child(
                        Label::new("Personal Information")
                            .text_size(AbsoluteLength::Pixels(px(34.0)))
                            .font_bold(),
                    )
                    .child(
                        v_flex().gap_3().pl_8().child(
                            v_form()
                                .child(
                                    field().child(
                                        h_flex()
                                            .gap_5()
                                            .items_center()
                                            .w_full()
                                            .child(
                                                field()
                                                    .label("Lastname")
                                                    .required(true)
                                                    .child(Input::new(&self.lastname).large()),
                                            )
                                            .child(
                                                field()
                                                    .label("Firstname")
                                                    .required(true)
                                                    .child(Input::new(&self.firstname).large()),
                                            ),
                                    ),
                                )
                                .child(
                                    field().child(
                                        h_flex()
                                            .gap_5()
                                            .items_center()
                                            .w_full()
                                            .child(
                                                field()
                                                    .label("Middlename")
                                                    .child(Input::new(&self.middlename).large()),
                                            )
                                            .child(
                                                field()
                                                    .label("Suffix (Optional)")
                                                    .child(Input::new(&self.suffix).large()),
                                            ),
                                    ),
                                )
                                .child(
                                    field()
                                        .label("Date of Birth")
                                        .child(DatePicker::new(&self.date_of_birth).large()),
                                ),
                        ),
                    )
                    .child(
                        Label::new("Contact Information")
                            .text_size(AbsoluteLength::Pixels(px(34.0)))
                            .font_bold(),
                    )
                    .child(
                        v_flex().gap_3().pl_8().child(
                            v_form()
                                .child(
                                    field()
                                        .label("Email Address")
                                        .required(true)
                                        .child(Input::new(&self.email).large()),
                                )
                                .child(
                                    field()
                                        .label("Phone Number")
                                        .required(true)
                                        .child(Input::new(&self.phone_number).large()),
                                )
                                .child(
                                    field()
                                        .label("Emergency Contact Name")
                                        .child(Input::new(&self.emergency_contact_name).large()),
                                )
                                .child(
                                    field()
                                        .label("Emergency Contact number")
                                        .child(Input::new(&self.emergency_contact_number).large()),
                                ),
                        ),
                    )
                    .child(
                        field().label_indent(false).child(
                            h_flex()
                                .gap_2()
                                .items_center()
                                .child(
                                    Button::new("submit-form-fields")
                                        .label("Create")
                                        .primary()
                                        .large()
                                        .cursor_pointer()
                                        .w(px(120.0)),
                                )
                                .child(
                                    Button::new("clear-form-fields")
                                        .label("Clear")
                                        .ghost()
                                        .cursor_pointer()
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.clear_input(window, cx)
                                        }))
                                        .w(px(120.0)),
                                ),
                        ),
                    ),
            )
    }
}

impl Render for CreateEmployee {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .gap_8()
            .child(self.render_header(cx))
            .child(self.render_form(window, cx))
    }
}
