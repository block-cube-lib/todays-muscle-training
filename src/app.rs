use crate::{components::*, date::Date, traning::*};
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use rand::rngs::ThreadRng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use yew::{prelude::*, Html};

const PAGE_TITLE: &str = "日替わり筋肉メイカー";
const CONTENT_TITLE: &str = "💪日替わり筋肉メイカー💪";
const SHARE_TAG: &str = PAGE_TITLE;

const LOCAL_STORAGE_KEY: &str = "daily_traning";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    traning_settings: Vec<TraningSetting>,
}

impl Config {
    fn load() -> Config {
        let config = include_str!("../config.toml");
        toml::from_str(&config).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyTraning {
    date: Date,
    traning_menus: Vec<Traning>,
}

impl DailyTraning {
    fn load_from_local_storage() -> DailyTraning {
        let storage_value: String = LocalStorage::get(LOCAL_STORAGE_KEY).unwrap_or_default();
        if let Ok(daily_traning) = serde_json::from_str::<DailyTraning>(&storage_value) {
            if daily_traning.date == Date::today() {
                daily_traning
            } else {
                Self {
                    date: Date::today(),
                    traning_menus: vec![],
                }
            }
        } else {
            Self {
                date: Date::today(),
                traning_menus: vec![],
            }
        }
    }

    fn write_to_local_storage(&self) {
        if let Ok(json) = serde_json::to_string(&self) {
            LocalStorage::set(LOCAL_STORAGE_KEY, json).ok();
        }
    }
}

pub enum AppMessage {
    Lottery,
    LoadChange(usize),
    OnDisplayHelpChange(bool),
}

pub struct App {
    pub daily_traning: DailyTraning,
    load: usize,
    rng: ThreadRng,
    config: Config,
    display_help: bool,
}

impl App {
    fn lottery_and_save(&mut self) {
        let today = Date::today();
        if self.daily_traning.date != today {
            self.daily_traning.traning_menus.clear();
            self.daily_traning.date = today;
        }

        let config = &self.config;
        let traning_settings_count = config.traning_settings.len();
        let traning_index: usize = self.rng.gen_range(0..traning_settings_count);
        let traning_setting = config.traning_settings[traning_index].clone();
        let traning = Traning {
            traning_setting,
            load: self.load,
        };
        self.daily_traning.traning_menus.push(traning);
        self.daily_traning.write_to_local_storage();
    }
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let config = Config::load();
        let daily_traning = DailyTraning::load_from_local_storage();
        let load = daily_traning
            .traning_menus
            .iter()
            .map(|x| x.load)
            .last()
            .unwrap_or(5);

        Self {
            daily_traning,
            load,
            rng: rand::thread_rng(),
            config,
            display_help: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        use AppMessage::*;
        match msg {
            Lottery => {
                self.lottery_and_save();
                true
            }
            LoadChange(load) => {
                self.load = load;
                true
            }
            OnDisplayHelpChange(display_help) => {
                self.display_help = display_help;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let header = html! { <Header display_help={self.display_help} on_display_help_changed={ link.callback(|x| AppMessage::OnDisplayHelpChange(x)) } /> };
        html! {
            <main>
                { header }
                { if self.display_help { self.view_help(&ctx) } else { self.view_main_content(&ctx) } }
            </main>
        }
    }
}

impl App {
    fn view_main_content(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        if let Some(traning) = &self.daily_traning.traning_menus.iter().last() {
            let share_text = format!("{}\n#{}\n", &traning, SHARE_TAG);
            html! {
                <div class="main_content">
                    <h1>{ CONTENT_TITLE }</h1>
                    <div class="traning_menu">{ format!("{}", &traning) }</div>
                    <div class="traning_description">{ format!("{}", traning.traning_setting.description()) }</div>
                    <div class="share_button_container">
                        <WebShareButton title={ PAGE_TITLE } text={ share_text.clone() } />
                        <TwitterShare text={ share_text.clone() } />
                    </div>
                </div>
            }
        } else {
            html! {
                <div class="main_content">
                    <h1>{ CONTENT_TITLE }</h1>
                    <LoadSelector value={ self.load } on_input = { link.callback(|load| AppMessage::LoadChange(load)) } />
                    <button class="lottery_button" onclick={ link.callback(|_| AppMessage::Lottery) }>{ "抽選する" }</button>
                </div>
            }
        }
    }

    fn view_help(&self, _ctx: &Context<Self>) -> Html {
        let traning_settings = &self.config.traning_settings;
        let push_up = traning_settings
            .iter()
            .find(|x| x.id() == "push_up")
            .unwrap();
        let full_squat = traning_settings
            .iter()
            .find(|x| x.id() == "full_squat")
            .unwrap();
        let sample_traning_name_and_amount = |name: &str, traning_setting: &TraningSetting, load1, load2, load3| {
            let traning1 = Traning {
                traning_setting: traning_setting.clone(),
                load: load1,
            };
            let traning2 = Traning {
                traning_setting: traning_setting.clone(),
                load: load2,
            };
            let traning3 = Traning {
                traning_setting: traning_setting.clone(),
                load: load3,
            };
            format!(
                "{}であれば\n負荷{}で{}、負荷{}で{}、負荷{}で{}",
                name,
                load1,
                traning1.amount_with_unit(),
                load2,
                traning2.amount_with_unit(),
                load3,
                traning3.amount_with_unit()
            )
        };
        html! {
            <div class="help">
                <h2>{ "使い方"}</h2>
                <li>{ "スライダーで筋トレの負荷を選ぶ" }</li>
                <li>{ "抽選ボタンを押す" }</li>
                <li>{ "決まった筋トレをする" }</li>
                { "以上です。" } <br/>
                { "抽選で決まる筋トレは自重でできる筋トレだけなのでダンベルなどの器具を持っていなくても大丈夫です。" }
                <hr/>
                <h2>{ "回数の例" }</h2>
                { sample_traning_name_and_amount("プッシュアップ(腕立て伏せ)", &push_up, 1, 5, 10) }{ "。" }<br/>
                { sample_traning_name_and_amount("フルスクワット", &full_squat, 1, 5, 10) } { "。" }<br/>
                { "といった具合に増えていきます。参考にしてください。" }
                <hr/>
                <h2>{ "ご意見・お問い合わせ" }</h2>
                { "作者のTwitterアカウント" } <a href="https://twitter.com/block_cube_lib">{"(@block_cube_lib)"}</a>{ "へお願いします。" }
            </div>
        }
    }
}
