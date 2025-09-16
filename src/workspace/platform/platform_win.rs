use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
    IconName, Sizable,
    button::{Button, ButtonVariants},
};

pub struct WindowsWindowControls;

impl WindowsWindowControls {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for WindowsWindowControls {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("windows-window-controls")
            .flex()
            .flex_row()
            .justify_center()
            .content_stretch()
            .child(
                Button::new("minimize")
                    .small()
                    .ghost()
                    .icon(IconName::WindowMinimize)
                    .map(|this| this.window_control_area(WindowControlArea::Min)),
            )
            .child(
                Button::new("maximize-or-restore")
                    .small()
                    .ghost()
                    .when_else(
                        window.is_maximized(),
                        |this| this.icon(IconName::WindowRestore),
                        |this| this.icon(IconName::WindowMaximize),
                    )
                    .map(|this| this.window_control_area(WindowControlArea::Max)),
            )
            .child(
                Button::new("close")
                    .small()
                    .ghost()
                    .icon(IconName::WindowClose)
                    .map(|this| this.window_control_area(WindowControlArea::Close)),
            )
    }
}
