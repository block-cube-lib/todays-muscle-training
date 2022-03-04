use yew::{function_component, Properties, Callback, prelude::*};

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct WebShareProperty {
    pub title: String,
    pub text: String,
}

#[function_component(WebShareButton)]
pub fn web_share_button(property: &WebShareProperty) -> Html {
    let property = property.clone();
    let callback = Callback::from(move |_|  crate::bindings::share(&property.title, &property.text) );
    html! {
        <button class="share_button" onclick={callback} >
            <img src="./assets/share_icon.png" id="share_image" height="40px"/>
        </button>
    }
}

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct TwitterShareProperty {
    pub text: String,
}

#[function_component(TwitterShare)]
pub fn twitter_share_button(property: &TwitterShareProperty) -> Html {
    let text = property.text.clone();
    html! {
        <div>
            <a href="https://twitter.com/share?ref_src=twsrc%5Etfw" class="twitter-share-button" data-text={ text } data-size="large" data-show-count="false">
                { "Tweet" }
            </a>
            <script src="https://platform.twitter.com/widgets.js" charset="utf-8"></script>
        </div>
    }
}
