use chrono::NaiveDate;
use yew::Component; 
use crate::*;
use yew_datepicker::Datepicker;

pub struct BackdateMoodReadings {
    pub current_date: Option<NaiveDate>,
    pub request_user_pick_date: bool,
}

pub enum BackdateMsg {
    DateSelected(NaiveDate),
    BackdateReading(MoodReading),
}

impl Component for BackdateMoodReadings {
    type Message = BackdateMsg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { current_date: None, request_user_pick_date: false }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            BackdateMsg::DateSelected(naive_date) => {
                self.current_date = Some(naive_date);
            }
            BackdateMsg::BackdateReading(reading) => {
                unimplemented!("Backdate reading");
            }
        }
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Datepicker on_select={ ctx.link()
                    .callback(|naive_date| 
                        BackdateMsg::DateSelected(naive_date))}/>

                { if self.request_user_pick_date {
                    html! {
                        <p> {"Please pick a date."} </p>
                    }
                } else {
                    html! {</>}
                }
            </>
        }
    }
}