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
}

pub struct App {
    pub daily_traning: DailyTraning,
    load: usize,
    rng: ThreadRng,
    config: Config,
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
        let load = daily_traning.traning_menus.iter().map(|x| x.load).last().unwrap_or(5);

        Self {
            daily_traning,
            load,
            rng: rand::thread_rng(),
            config,
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(traning) = &self.daily_traning.traning_menus.iter().last() {
            let share_text = format!("{}\n#{}\n", &traning, SHARE_TAG);
            html! {
                <div>
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
            let link = ctx.link();
            html! {
                <main>
                    <h1>{ CONTENT_TITLE }</h1>
                    <LoadSelector value={ self.load } on_input = { link.callback(|load| AppMessage::LoadChange(load)) } />
                    <button class="lottery_button" onclick={ link.callback(|_| AppMessage::Lottery) }>{ "抽選する" }</button>
                </main>
            }
        }
    }
}
