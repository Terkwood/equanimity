use yew::Component;
use yew_datepicker::Datepicker;
use crate::*;

pub struct BackdateMoodReadings {

}

impl Component for BackdateMoodReadings {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Datepicker on_select={ ctx.link().callback(|naive_date| unimplemented!())}/>
        }
    }
}