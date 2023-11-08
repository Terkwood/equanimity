
use crate::*;
pub struct History {
    link: ComponentLink<Self>,
    props: HistoryProps
}

#[derive(Properties, Clone, PartialEq)]
pub struct HistoryProps{}

pub struct HistoryMsg ;

impl Component for History {
    type Message = HistoryMsg;
    type Properties = HistoryProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
         

        Self {
            link,
            
            props,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            HistoryMsg => {
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }
    fn view(&self) -> Html {
        
            html! { <>
                <div id="history">
                    <p>{ "OK THEN" }</p>
                </div>
            </> }
        }
    }