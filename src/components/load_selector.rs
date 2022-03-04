use yew::{prelude::*, Callback, Component, InputEvent, Properties};
use web_sys::HtmlInputElement;

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct LoadSelectorProperty {
    pub value: usize,
    pub on_input: Callback<usize>,
}

pub struct LoadSelector {
    pub load: usize,
    pub on_input: Callback<usize>,
}

pub enum LotterySelectorMessage {
    OnInput(usize),
}

impl Component for LoadSelector {
    type Message = LotterySelectorMessage;
    type Properties = LoadSelectorProperty;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        Self {
            load: props.value,
            on_input: props.on_input,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let load = ctx.props().value;
        let on_input = ctx.link().callback(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            let load = if let Some(Ok(load)) = input.map(|x| x.value().parse()) {
                load
            } else {
                load
            };
            LotterySelectorMessage::OnInput(load)
        });
        let load = format!("{}", load);
        html! {
            <div class="load_selector">
                { "負荷: " }{ self.load }
                <p><input type="range" class="load_selector_input" value={ load } name="load" min="1" max="10" step="1" oninput={ on_input } /></p>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use LotterySelectorMessage::*;
        match msg {
            OnInput(load) => {
                self.load = load;
                self.on_input.emit(load);
            },
        }
        true
    }
}
