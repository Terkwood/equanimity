use chrono::NaiveDate;
use yew::Component; 
use crate::*;
use yew_datepicker::Datepicker;

use super::storage_state::StorageState;

pub struct BackdateMoodReadings {
    pub mood_reading: Option<MoodReading>,
    pub current_date: Option<NaiveDate>,
}

pub enum BackdateMsg {
    DateSelected(NaiveDate),
    MoodReadingSelected(MoodReading),
    BackdateReading(NaiveDate, MoodReading),
}

impl Component for BackdateMoodReadings {
    type Message = BackdateMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { mood_reading: None, current_date: None, }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BackdateMsg::MoodReadingSelected(mood_reading) => {
                self.mood_reading = Some(mood_reading);
            }
            BackdateMsg::DateSelected(naive_date) => {
                self.current_date = Some(naive_date);
            }
            BackdateMsg::BackdateReading(date, reading) => {
                web_sys::console::log_1(&format!("Backdating mood reading {:?} to {:?}", reading, date).into());
            }
        }
        
        true
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
                    if let Some(mr) = &self.mood_reading {
                        if let Some(d) = &self.current_date {
                            let mood_reading = mr.clone();
                            let date = d.clone();
                            html!{
                                <div class="backdate-child">
                                <button 
                                    class="fancy-button" 
                                    role="button" 
                                    onclick={
                                        ctx
                                            .link()
                                            .callback(move |_| BackdateMsg::BackdateReading(date.clone(), mood_reading.clone()))}>
                                    { "Backdate" }
                                </button>
                            </div>
                            }
                        } else {
                            html! { <> </> }
                        }
                    } else { 
                        html! { <> </> } 
                    }
                }
                
            </div>
            </>
        }
    }
}
