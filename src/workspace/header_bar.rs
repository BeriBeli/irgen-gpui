use gpui::*;
use gpui_component::{
    ActiveTheme as _, Icon, IconName, Sizable,
    button::{Button, ButtonVariants},
    input::{InputState, TextInput},
    label::Label,
    popup_menu::PopupMenuExt,
};

pub struct HeaderBar {
    search_prompt: Entity<InputState>,
}

impl HeaderBar {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_prompt = cx.new(|cx| InputState::new(window, cx).placeholder("Search"));
        Self { search_prompt }
    }
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for HeaderBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let menu = Button::new("header-menu")
            .small()
            .ghost()
            .compact()
            .icon(IconName::Menu)
            .outline()
            .popup_menu(move |this, _, _| {
                this.menu_with_icon(
                    "export IP-XACT",
                    IconName::ExternalLink,
                    Box::new(gpui_component::input::Copy), // TODO
                )
            });
        let search = TextInput::new(&self.search_prompt)
            .cleanable()
            .prefix(Icon::new(IconName::Search).small());

        div()
            .id("header-bar")
            .bg(cx.theme().background)
            .border_color(cx.theme().border)
            .border_b_1()
            .p_1()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .child(Label::new("irgen").text_xs().pl(px(5.0)))
                    .child(div().h_auto().w_auto().min_w_64().child(search))
                    .child(div().pr(px(5.0)).flex().items_center().child(menu)),
            )
    }
}
