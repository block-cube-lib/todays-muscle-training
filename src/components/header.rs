use yew::{prelude::*, Callback, Component, Properties};

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct HeaderProperty {
    pub display_help: bool,
    pub on_display_help_changed: Callback<bool>,
}

pub struct Header {
    pub display_help: bool,
    pub on_display_help_changed: Callback<bool>,
}

pub enum HeaderMessage {
    OnDisplayHelpChanged,
}

impl Component for Header {
    type Message = HeaderMessage;
    type Properties = HeaderProperty;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        Self {
            display_help: props.display_help,
            on_display_help_changed: props.on_display_help_changed,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_help_button_click = ctx.link().callback(|_| HeaderMessage::OnDisplayHelpChanged);
        let icon_path = if self.display_help {
            "assets/close_icon.png"
        } else {
            "assets/question_icon.png"
        };
        html! {
            <header>
                <button onclick={ on_help_button_click }><img src={ icon_path }/></button>
            </header>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use HeaderMessage::*;
        match msg {
            OnDisplayHelpChanged => {
                self.display_help = !self.display_help;
                self.on_display_help_changed.emit(self.display_help)
            },
        }
        false
    }
}
