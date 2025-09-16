#[cfg(target_os = "windows")]
use super::platform::platform_win::WindowsWindowControls;
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::label::Label;
use gpui_component::{
    ActiveTheme as _, ContextModal as _, IconName, Sizable as _, ThemeMode,
    badge::Badge,
    button::{Button, ButtonVariants as _},
};

use crate::themes::*;

#[cfg(target_os = "macos")]
const TITLE_BAR_LEFT_PADDING: Pixels = px(80.);
#[cfg(target_os = "windows")]
const TITLE_BAR_LEFT_PADDING: Pixels = px(5.);

pub struct TitleBar {
    #[cfg(target_os = "windows")]
    controls: Entity<WindowsWindowControls>,
}

impl TitleBar {
    #[cfg(target_os = "windows")]
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            controls: WindowsWindowControls::view(window, cx),
        }
    }
    #[cfg(target_os = "macos")]
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
    pub fn change_mode(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        println!("Current mode: {:?}", cx.theme().mode);
        let new_mode = if cx.theme().mode.is_dark() {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        };
        change_color_mode(new_mode, window, cx);
    }
}

impl Render for TitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notifications_count = window.notifications(cx).len();
        let theme_toggle = Button::new("theme-mode")
            .map(|this| {
                if cx.theme().mode.is_dark() {
                    this.icon(IconName::Sun)
                } else {
                    this.icon(IconName::Moon)
                }
            })
            .small()
            .ghost()
            .on_click(cx.listener(Self::change_mode));

        let github_button = Button::new("github")
            .icon(IconName::GitHub)
            .small()
            .ghost()
            .on_click(|_, _, cx| cx.open_url("https://github.com/BeriBeli/irgen-gpui"));

        let notification_button = Badge::new().count(notifications_count).max(99).child(
            Button::new("bell")
                .small()
                .ghost()
                .compact()
                .icon(IconName::Bell),
        );

        #[cfg(target_os = "macos")]
        {
            div()
                .id("title-bar")
                .border_b_1()
                .bg(cx.theme().title_bar)
                .border_color(cx.theme().border)
                .pl(TITLE_BAR_LEFT_PADDING)
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .items_center()
                        .p_1()
                        .child(Label::new("irgen").text_xs())
                        .child(
                            div()
                                .pr(px(5.0))
                                .flex()
                                .items_center()
                                .child(theme_toggle)
                                .child(github_button)
                                .child(notification_button),
                        ),
                )
        }
        #[cfg(target_os = "windows")]
        {
            div()
                .id("title-bar")
                .flex()
                .justify_between()
                .border_b_1()
                .bg(cx.theme().title_bar)
                .border_color(cx.theme().border)
                .pl(TITLE_BAR_LEFT_PADDING)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .p_1()
                        .child(Label::new("irgen").text_xs())
                        .map(|this| this.window_control_area(WindowControlArea::Drag)),
                )
                .child(
                    div()
                        .flex_grow()
                        .map(|this| this.window_control_area(WindowControlArea::Drag)),
                )
                .child(
                    div()
                        .pr(px(5.0))
                        .flex()
                        .items_center()
                        .child(theme_toggle)
                        .child(github_button)
                        .child(notification_button)
                        .child(self.controls.clone()),
                )
        }
    }
}
