use gpui::*;
use gpui_component::ActiveTheme as _;

pub struct SideBar {}

impl SideBar {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for SideBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("side-bar")
            .flex()
            .h_full()
            .bg(cx.theme().background)
            .border_color(cx.theme().border)
            .border_r_1()
            .min_w(px(200.0))
    }
}
