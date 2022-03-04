use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

impl Date {
    pub fn today() -> Self {
        let js_date = js_sys::Date::new_0();
        js_date.set_time(js_sys::Date::now());
        let year = js_date.get_full_year();
        let month = js_date.get_month() + 1;
        let day = js_date.get_date();
        Self { year, month, day }
    }
}