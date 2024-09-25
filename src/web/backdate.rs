use crate::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use yew::Component;
use yew_datepicker::Datepicker;

pub struct BackdateMoodReadings {
    pub mood_reading: Option<MoodReading>,
    pub current_date: Option<NaiveDate>,
}

pub enum BackdateMsg {
    DateSelected(NaiveDate),
    MoodReadingSelected(MoodReading),
    BackdateReading,
    ShowHome,
}

#[derive(Properties, Clone, PartialEq)]
pub struct BackdateProps {
    pub add_mood_reading: Callback<MoodReading>,
    pub show_home: Callback<()>,
}

impl Component for BackdateMoodReadings {
    type Message = BackdateMsg;
    type Properties = BackdateProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            mood_reading: None,
            current_date: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BackdateMsg::ShowHome => {
                ctx.props().show_home.emit(());
                false
            }
            BackdateMsg::MoodReadingSelected(mood_reading) => {
                self.mood_reading = Some(mood_reading);
                true
            }
            BackdateMsg::DateSelected(naive_date) => {
                self.current_date = Some(naive_date);
                true
            }
            BackdateMsg::BackdateReading => {
                if let Some(mood_reading) = self.mood_reading {
                    if let Some(date) = self.current_date {
                        let draft_offset =
                            js_sys::Date::new(&JsValue::from_f64(mood_reading.epoch_millis as f64))
                                .get_timezone_offset() as i64;

                        web_sys::console::log_1(&format!("draft_offset: {}", draft_offset).into());

                        let naive_datetime = NaiveDateTime::new(
                            date,
                            NaiveTime::from_hms_opt(draft_offset as u32 / 60, 0, 0).unwrap(),
                        );

                        ctx.props().add_mood_reading.emit(MoodReading {
                            value: mood_reading.value,
                            epoch_millis: naive_datetime.and_utc().timestamp_millis() as u64,
                        });
                    }
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div id="mood-button-grid">
                <div class="center">
                    <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| BackdateMsg::MoodReadingSelected(MoodReading::new(-3)))}>{ "🏥 3️⃣ 🏥" }</button>
                </div>
                <div class="center">
                    <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| BackdateMsg::MoodReadingSelected(MoodReading::new(-2)))}>{ "😭 2️⃣ 😭" }</button>
                </div>
                <div class="center">
                    <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| BackdateMsg::MoodReadingSelected(MoodReading::new(-1)))}>{ "😢 1️⃣ 😢" }</button>
                </div>
                <div class="center">
                    <button id="equanimity-button" class="fancy-button" onclick={ctx.link().callback(|_| BackdateMsg::MoodReadingSelected(MoodReading::new(0)))}>{ "☯" }</button>
                </div>
                <div class="center">
                    <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| BackdateMsg::MoodReadingSelected(MoodReading::new(1)))}>{ "⚡ 1️⃣ ⚡" }</button>
                </div>
                <div class="center">
                    <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| BackdateMsg::MoodReadingSelected(MoodReading::new(2)))}>{ "🔥 2️⃣ 🔥" }</button>
                </div>
                <div class="center">
                    <button class="fancy-button mood-button" role="button" onclick={ctx.link().callback(|_| BackdateMsg::MoodReadingSelected(MoodReading::new(3)))}>{ "🤯 3️⃣ 🤯" }</button>
                </div>
            </div>

            <div id="backdate">
                <div class="backdate-child">
                <Datepicker on_select={ ctx.link()
                    .callback(|naive_date|
                        BackdateMsg::DateSelected(naive_date))}/>
                </div>
                {
                    if let Some(mr) = self.mood_reading {
                        html! {
                            <div class="day-container backdate-child">
                                <div class="piplabel">{ pips::blank_label() }</div>
                                <div class="pips">{ pips::circles(&[mr.value]) }</div>
                                <div class="piplabel">{ pips::blank_label() }</div>
                            </div>
                        }
                    } else { html! { <></> } }
                }


                <div class="backdate-child">
                {
                    if let Some(d) =  self.current_date {
                        format!("Date selected: {:?}",d)
                    } else {
                        "Please, select a date.".to_string()
                    }
                }
                </div>


                {
                    if  self.mood_reading.is_some() && self.current_date.is_some(){
                            html!{
                                <div class="backdate-child">
                                <button
                                    class="fancy-button thick"
                                    role="button"
                                    onclick={
                                        ctx
                                            .link()
                                            .callback(move |_| BackdateMsg::BackdateReading)}>
                                    { "Backdate" }
                                </button>
                            </div>
                            }
                    } else {
                        html! { <> { "Please, select a mood." }  </> }
                    }
                }

                <div class="backdate-child">
                    <button class="fancy-button thick"
                        role="button"
                        onclick={ctx.link().callback(|_| BackdateMsg::ShowHome) }>
                        { "Home" }
                    </button>
                </div>

            </div>
            </>
        }
    }
}
