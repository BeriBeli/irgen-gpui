use anyhow::Result;
use gpui::{
    Application, AssetSource, Bounds, Context, Menu, MenuItem, SharedString, TitlebarOptions,
    Window, WindowBounds, WindowKind, WindowOptions, div, prelude::*, px, rgb, size, svg,
};
use gpui_component::{
    Root, StyledExt,
    button::{Button, ButtonVariants},
};
use std::fs;
use std::path::PathBuf;

struct Assets {
    base: PathBuf,
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }
}

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(rgb(0xf9fafb))
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
                        .child(
                            div()
                                .font_bold()
                                .text_3xl()
                                .text_center()
                                .mb_8()
                                .child("irgen"),
                        )
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
                                .border_color(rgb(0x93c5fd))
                                .hover(|this| this.bg(rgb(0xeff6ff)).text_color(rgb(0x2563eb)))
                                .cursor_pointer()
                                .child(
                                    svg()
                                        .path("assets/excel.svg")
                                        .w_12()
                                        .h_12()
                                        .text_color(rgb(0x3b82f6)),
                                )
                                .child("Click to select a spreadsheet")
                                .on_click(|_, _, _| println!("Select a file!")),
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
                                        .on_click(|_, _, _| println!("Exported ipxact!"))
                                        .cursor_pointer(),
                                )
                                .child(
                                    Button::new("button1")
                                        .primary()
                                        .items_center()
                                        .label("export regvue")
                                        .on_click(|_, _, _| println!("Exported regvue!"))
                                        .cursor_pointer(),
                                ),
                        ),
                ),
            )
    }
}

fn main() {
    let app = Application::new().with_assets(Assets {
        base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    });

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.activate(true);

        let mut window_size = size(px(800.), px(600.));
        if let Some(display) = cx.primary_display() {
            let display_size = display.bounds().size;
            window_size.width = window_size.width.min(display_size.width * 0.85);
            window_size.height = window_size.height.min(display_size.height * 0.85);
        }
        let window_bounds = Bounds::centered(None, window_size, cx);

        cx.set_menus(vec![Menu {
            name: "irgen".into(),
            items: vec![MenuItem::Separator],
        }]);

        cx.spawn(async move |cx| {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(window_bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Default::default(),
                    appears_transparent: Default::default(),
                    traffic_light_position: Default::default(),
                }),
                window_min_size: Some(gpui::Size {
                    width: px(800.),
                    height: px(600.),
                }),
                kind: WindowKind::Normal,
                #[cfg(target_os = "linux")]
                window_background: gpui::WindowBackgroundAppearance::Transparent,
                #[cfg(target_os = "linux")]
                window_decorations: Some(gpui::WindowDecorations::Client),
                ..Default::default()
            };

            let window = cx
                .open_window(options, |window, cx| {
                    let view = cx.new(|_| HelloWorld);
                    cx.new(|cx| Root::new(view.into(), window, cx))
                })
                .expect("failed to open window");

            window
                .update(cx, |_, window, _| {
                    window.activate_window();
                    window.set_window_title("irgen");
                })
                .expect("failed to update window");

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
