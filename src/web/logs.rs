use super::State;
use crate::*;
use repo::YewRepo;
use std::rc::Rc;

pub struct Logs {
    link: ComponentLink<Self>,
    repo: YewRepo,
    state: State,
    show_bars: Callback<()>,
}

pub enum LogsMsg {
    ShowBars,
}

#[derive(Properties, Clone)]
pub struct LogsProps {
    pub show_bars: Callback<()>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Entries {
    Mood(MoodReading),
    Sleep(TextSubmission),
    Note(TextSubmission),
}

impl Entries {
    pub fn timestamp(&self) -> u64 {
        match self {
            Entries::Mood(m) => m.epoch_millis,
            Entries::Sleep(t) => t.epoch_millis,
            Entries::Note(t) => t.epoch_millis,
        }
    }
}

/*impl PartialEq for Entries {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp() == other.timestamp()
    }
}*/

impl Component for Logs {
    type Message = LogsMsg;
    type Properties = LogsProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let repo = YewRepo::new();

        let state = State::load(&repo);

        Self {
            link,
            state,
            repo,
            show_bars: props.show_bars,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            LogsMsg::ShowBars => {
                self.show_bars.emit(());
                false
            }
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }
    fn view(&self) -> Html {
        html! {
            <button onclick=self.link.callback(|_| LogsMsg::ShowBars)>{ "Show Bars 📊"}</button>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_sort_entries() {
        let should_be_first = Entries::Sleep(TextSubmission {
            value: "hello".to_string(),
            epoch_millis: 0,
        });

        let should_be_middle = Entries::Note(TextSubmission {
            value: "yes".to_string(),
            epoch_millis: 50,
        });

        let should_be_last = Entries::Mood(MoodReading {
            value: 0,
            epoch_millis: 100,
        });

        let mut entries = vec![should_be_last, should_be_first, should_be_middle];

        entries.sort();
    }
}
