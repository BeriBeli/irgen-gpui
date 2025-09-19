use super::header_bar::HeaderBar;
use super::side_bar::SideBar;
use super::table::RegField;
use super::title_bar::TitleBar;

use gpui::prelude::*;
use gpui::*;
use gpui_component::Root;
// use std::sync::Arc;

use gpui_component::{
    ActiveTheme as _,
    // button::{Button, ButtonVariants as _},
};

pub struct Workspace {
    title_bar: Entity<TitleBar>,
    header_bar: Entity<HeaderBar>,
    side_bar: Entity<SideBar>,
    table: Entity<RegField>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let title_bar = TitleBar::view(window, cx);
        let header_bar = HeaderBar::view(window, cx);
        let side_bar = SideBar::view(window, cx);
        let table = RegField::view(window, cx);

        Self {
            title_bar,
            header_bar,
            side_bar,
            table,
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notification_layer = Root::render_notification_layer(window, cx);

        let main = div().id("workspace-main").child(self.table.clone());

        let content = div()
            .id("workspace-content")
            .flex()
            .flex_grow()
            .bg(cx.theme().background)
            .child(self.side_bar.clone())
            .child(main);

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.title_bar.clone())
            .child(self.header_bar.clone())
            .child(content)
            .children(notification_layer)
    }
}
