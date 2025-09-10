use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size, svg,
};

struct HelloWorld {}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(rgb(0xf9fafb))
            .text_color(rgb(0x1f2937))
            .flex()
            .items_center()
            .justify_center()
            .min_h_full()
            .w_full()
            .child(
                div()
                    .w_full()
                    .max_w(px(672.0))
                    .mx_auto()
                    .p_4()
                    .child(
                        div()
                            .bg(gpui::white())
                            .rounded_xl()
                            .shadow_lg()
                            .p_8()
                            .child(
                                div()
                                    .text_center()
                                    .mb_8()
                                    .child(
                                        "irgen"
                                    )
                            )
                            .child(
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .px_4()
                                    .py_10()
                                    .bg(gpui::white())
                                    .text_color(rgb(0x3b82f6))
                                    .rounded_lg()
                                    .shadow_md()
                                    .border(px(1.0))
                                    .border_dashed()
                                    .border_color(rgb(0x93c5fd))
                                    // .hover(|this| this.bg(rgb(0xeff6ff)))
                                    // .hover(|this| this.text_color(rgb(0x2563eb)))
                                    .cursor_pointer()
                                    .child(
                                        div()
                                            .child(
                                                svg()
                                                    .w_12()
                                                    .h_12()
                                                    .text_color(rgb(0x3b82f6))
                                                    .path("M6 2a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7.414A2 2 0 0 0 15.414 6L12 2.586A2 2 0 0 0 10.586 2H6zm1 8h3v2H7v-2zm0 4h6v2H7v-2zm4-4h3v2h-3v-2z"
                                                    )
                                            )
                                    )
                            )
                            .child(
                                div()
                                    .grid()
                                    .grid_cols(1)
                                    .gap_4()
                                    .child(
                                        "export ipxact"
                                    )
                                    .child(
                                        "export regvue"
                                    )
                            )
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| HelloWorld {}),
        )
        .unwrap();
    });
}
