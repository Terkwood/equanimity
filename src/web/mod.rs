mod about;
mod backdate;
mod history;
pub mod logs;
pub mod storage_state;
pub mod time;

use crate::*;
use backdate::BackdateMoodReadings;
use history::History;
use logs::Logs;
use storage_state::StorageState;

const INITIAL_MODE: Mode = Mode::BackdateMoodReadings;

pub struct Root {
    mode: Mode,
    storage_state: storage_state::StorageState,
    show_logs: Option<Callback<()>>,
    show_history: Option<Callback<()>>,
    add_mood_reading: Option<Callback<MoodReading>>,
    add_text: Option<Callback<(TextType, String)>>,
    replace_texts: Option<Callback<(TextType, Vec<TextSubmission>)>>,
    replace_mood_readings: Option<Callback<Vec<MoodReading>>>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    Logs,
    History,
    BackdateMoodReadings,
}

pub enum RootMsg {
    SwitchMode(Mode),
    AddMoodReading(MoodReading),
    AddText(TextType, String),
    ReplaceMoodReadings(Vec<MoodReading>),
    ReplaceTexts(TextType, Vec<TextSubmission>),
}

impl Component for Root {
    type Message = RootMsg;
    type Properties = ();
    fn create(ctx: &yew::Context<Self>) -> Self {
        let show_logs = Some(ctx.link().callback(|()| RootMsg::SwitchMode(Mode::Logs)));
        let show_history = Some(ctx.link().callback(|()| RootMsg::SwitchMode(Mode::History)));

        let add_text = Some(
            ctx.link()
                .callback(|(text_type, text)| RootMsg::AddText(text_type, text)),
        );
        let add_mood_reading = Some(
            ctx.link()
                .callback(|mood_reading| RootMsg::AddMoodReading(mood_reading)),
        );
        let replace_texts = Some(
            ctx.link()
                .callback(|(text_type, text)| RootMsg::ReplaceTexts(text_type, text)),
        );
        let replace_mood_readings = Some(
            ctx.link()
                .callback(|readings| RootMsg::ReplaceMoodReadings(readings)),
        );

        let storage_state = StorageState::load();

        Self {
            mode: INITIAL_MODE,
            storage_state,
            show_logs,
            show_history,
            add_mood_reading,
            add_text,
            replace_texts,
            replace_mood_readings,
        }
    }
    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RootMsg::SwitchMode(new_mode) => {
                let old = self.mode;
                self.mode = new_mode;
                self.mode != old
            }
            RootMsg::AddText(TextType::Sleep, text) => {
                self.storage_state
                    .sleep_entries
                    .push(TextSubmission::new(text));

                repo::save_text(TextType::Sleep, &self.storage_state.sleep_entries)
                    .expect("save sleep");

                true
            }
            RootMsg::AddText(TextType::Meds, text) => {
                self.storage_state.meds.push(TextSubmission::new(text));
                repo::save_text(TextType::Meds, &self.storage_state.meds).expect("save meds");
                true
            }
            RootMsg::AddText(TextType::Notes, text) => {
                self.storage_state.notes.push(TextSubmission::new(text));
                repo::save_text(TextType::Notes, &self.storage_state.notes).expect("save notes");
                true
            }
            RootMsg::AddMoodReading(value) => {
                self.storage_state.mood_readings.push(value);
                repo::save_mood_readings(&self.storage_state.mood_readings)
                    .expect("save mood readings");
                true
            }
            RootMsg::ReplaceMoodReadings(readings) => {
                self.storage_state.mood_readings = readings.clone();
                repo::save_mood_readings(&readings).expect("replace mood readings");
                true
            }
            RootMsg::ReplaceTexts(text_type, all) => {
                match text_type {
                    TextType::Meds => self.storage_state.meds = all.clone(),
                    TextType::Notes => self.storage_state.notes = all.clone(),
                    TextType::Sleep => self.storage_state.sleep_entries = all.clone(),
                };
                repo::save_text(text_type, &all).expect("replace text entries");
                true
            }
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        match self.mode {
            Mode::BackdateMoodReadings => html! {
                <BackdateMoodReadings
                    add_mood_reading={self.add_mood_reading.as_ref().expect("add mood reading cb backdate")}
                    show_history={self.show_history.as_ref().expect("history cb")}
                />
            },
            Mode::Logs => html! {
                <Logs
                    storage_state={self.storage_state.clone()}
                    show_history={self.show_history.as_ref().expect("history cb")}
                    replace_mood_readings={self.replace_mood_readings.as_ref().expect("rmr_cb")}
                    replace_texts={self.replace_texts.as_ref().expect("rt_cb")}
                />
            },
            Mode::History => html! {
                <History
                    storage_state={self.storage_state.clone()}
                    show_logs={self.show_logs.as_ref().expect("logs_cb")}
                    add_mood_reading={self.add_mood_reading.as_ref().expect("smrcb")}
                    add_text={self.add_text.as_ref().expect("smtcb")}
                />
            },
        }
    }
}
