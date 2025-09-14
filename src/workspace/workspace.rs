use super::action::{open, save};
use super::header_bar::HeaderBar;

use crate::services::*;
use gpui::*;
use std::sync::Arc;

use gpui_component::{
    ActiveTheme,
    button::{Button, ButtonVariants},
};

pub struct Workspace {
    header_bar: Entity<HeaderBar>,
    app_state: Arc<AppState>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let header_bar = HeaderBar::view(window, cx);

        Self {
            header_bar,
            app_state: Arc::new(AppState::new()),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let app_state = self.app_state.clone();
        let main = div()
            .id("workspace-main")
            .bg(cx.theme().background)
            .text_color(rgb(0x1f2937))
            .flex()
            .items_center()
            .justify_center()
            .h_full()
            .w_full()
            .child(
                div().w_full().max_w(px(672.0)).mx_auto().p_4().child(
                    div()
                        .bg(gpui::white())
                        .rounded_xl()
                        .shadow_lg()
                        .p_8()
                        .child(div().text_3xl().text_center().mb_8().child("irgen"))
                        .child(
                            div()
                                .id("file-upload")
                                .w_full()
                                .flex()
                                .flex_col()
                                .items_center()
                                .px_4()
                                .py_10()
                                .mb_12()
                                .bg(gpui::white())
                                .text_color(rgb(0x3b82f6))
                                .rounded_lg()
                                .shadow_md()
                                .border(px(1.0))
                                .border_dashed()
                                .border_color(cx.theme().border)
                                .hover(|this| this.bg(rgb(0xeff6ff)).text_color(rgb(0x2563eb)))
                                .cursor_pointer()
                                .child(
                                    svg()
                                        .path("icons/excel.svg")
                                        .w_12()
                                        .h_12()
                                        .text_color(rgb(0x3b82f6)),
                                )
                                .child("Click to select a spreadsheet")
                                .on_click({
                                    let app_state = app_state.clone();
                                    move |_, _, app| open(app_state.clone(), load_excel, app)
                                }),
                        )
                        .child(
                            div()
                                .grid()
                                .grid_cols(1)
                                .justify_center()
                                .flex()
                                .gap_32()
                                .child(
                                    Button::new("button0")
                                        .primary()
                                        .items_center()
                                        .label("export ipxact")
                                        .on_click({
                                            let app_state = app_state.clone();
                                            move |_, _, app| {
                                                save(app_state.clone(), export_ipxact_xml, app)
                                            }
                                        })
                                        .cursor_pointer(),
                                )
                                .child(
                                    Button::new("button1")
                                        .primary()
                                        .items_center()
                                        .label("export regvue")
                                        .on_click({
                                            let app_state = app_state.clone();
                                            move |_, _, app| {
                                                save(app_state.clone(), export_regvue_json, app)
                                            }
                                        })
                                        .cursor_pointer(),
                                ),
                        ),
                ),
            );

        let content = div()
            .id("workspace-content")
            .flex()
            .flex_grow()
            .bg(cx.theme().background)
            .child(main);

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.header_bar.clone())
            .child(content)
    }
}
