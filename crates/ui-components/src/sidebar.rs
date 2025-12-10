use gpui::{prelude::FluentBuilder, *};
use gpui_component::{ActiveTheme, StyledExt};

const DEFAULT_WIDTH: Pixels = px(320.);
const COLLAPSED_WIDTH: Pixels = px(100.);

#[derive(IntoElement)]
pub struct SidebarComponent<E: IntoElement + 'static> {
    style: StyleRefinement,
    content: Vec<E>,
    /// sidebar header view
    header: Option<AnyElement>,
    /// sidebar footer view
    footer: Option<AnyElement>,
    collapsible: bool,
    collapsed: bool,
}

impl<E: IntoElement> SidebarComponent<E> {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            content: vec![],
            header: None,
            footer: None,
            collapsible: true,
            collapsed: false,
        }
    }

    /// Set the sidebar to be collapsible, default is true
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Set the sidebar to be collapsed
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Set the header of the sidebar.
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    /// Set the footer of the sidebar.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    /// Add a child element to the sidebar, the child must implement `Collapsible`
    pub fn child(mut self, child: E) -> Self {
        self.content.push(child);
        self
    }

    /// Add multiple children to the sidebar, the children must implement `Collapsible`
    pub fn children(mut self, children: impl IntoIterator<Item = E>) -> Self {
        self.content.extend(children);
        self
    }
}

impl<E: IntoElement> Styled for SidebarComponent<E> {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        &mut self.style
    }
}

impl<E: IntoElement> RenderOnce for SidebarComponent<E> {
    fn render(mut self, _window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        self.style.padding = EdgesRefinement::default();

        div()
            .v_flex()
            .id("sidebar-component")
            .w(DEFAULT_WIDTH)
            .flex_shrink_0()
            .h_full()
            .overflow_hidden()
            .relative()
            .bg(cx.theme().sidebar)
            .text_color(cx.theme().sidebar_foreground)
            .refine_style(&self.style)
            .when(self.collapsed, |this| this.w(COLLAPSED_WIDTH).gap_2())
            .when_some(self.header.take(), |this, header| {
                this.child(div().h_flex().id("sidebar-header").p_3())
                    .items_center()
                    .gap_2()
                    .when(self.collapsed, |this| this.p_2())
                    .child(header)
            })
            .child(
                div()
                    .v_flex()
                    .id("sidebar-content")
                    .flex_1()
                    .min_h_0()
                    .child(
                        div()
                            .v_flex()
                            .id("inner-content")
                            .p_3()
                            .when(self.collapsed, |this| this.p_2())
                            .children(self.content)
                            .overflow_y_scroll(),
                    ),
            )
            .when_some(self.footer.take(), |this, footer| {
                this.child(div().h_flex().id("sidebar-footer").pb_3().px_3())
                    .when(self.collapsed, |this| this.pt_2().px_2())
                    .child(footer)
            })
    }
}
