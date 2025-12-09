use gpui::*;

pub struct HeadTitleBar;

impl HeadTitleBar {
    pub fn new(window: &mut Window, ctx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn view(window: &mut Window, ctx: &mut App) -> Entity<Self> {
        ctx.new(build_entity)
    }
}
