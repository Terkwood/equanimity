use crate::*;

pub struct Model {
    link: ComponentLink<Self>,
    mood_readings: Vec<MoodReading>,
    sleep_entries: Vec<TextSubmission>,
    sleep_text_area: String,
    notes: Vec<TextSubmission>,
    notes_text_area: String,
}

pub enum Msg {
    AddReading(MoodReading),
    SleepTextAreaUpdated(String),
    SubmitSleep,
    NotesTextAreaUpdated(String),
    SubmitNotes,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            mood_readings: vec![],
            sleep_entries: vec![],
            sleep_text_area: "".to_string(),
            notes: vec![],
            notes_text_area: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddReading(r) => {
                self.mood_readings.push(r);
                repo::save_mood_readings(&self.mood_readings).expect("save mood readings")
            }
            Msg::SleepTextAreaUpdated(s) => self.sleep_text_area = s,
            Msg::SubmitSleep => {
                if !self.sleep_text_area.is_empty() {
                    self.sleep_entries
                        .push(TextSubmission::new(self.sleep_text_area.clone()));
                    self.sleep_text_area = "".to_string();
                    repo::save_sleep(&self.sleep_entries).expect("save sleep")
                }
            }
            Msg::NotesTextAreaUpdated(s) => self.notes_text_area = s,
            Msg::SubmitNotes => {
                if !self.notes_text_area.is_empty() {
                    self.notes
                        .push(TextSubmission::new(self.notes_text_area.clone()));
                    self.notes_text_area = "".to_string();
                    repo::save_notes(&self.notes).expect("save notes")
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div id="controlgrid">
                    <div>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(MoodReading::new(3)))>{ "🤯 3 🤯" }</button>
                        <br/>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(MoodReading::new(2)))>{ "🔥 2 🔥" }</button>
                        <br/>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(MoodReading::new(1)))>{ "⚡ 1 ⚡" }</button>
                        <br/>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(MoodReading::new(0)))>{ "☯ 🧘 ☯" }</button>
                        <br/>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(MoodReading::new(-1)))>{ "😢 1 😢" }</button>
                        <br/>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(MoodReading::new(-2)))>{ "😭 2 😭" }</button>
                        <br/>
                        <button class="moodbutton" onclick=self.link.callback(|_| Msg::AddReading(MoodReading::new(-3)))>{ "🏥 3 🏥" }</button>


                    </div>

                    <div>
                        <textarea
                            rows=6
                            value=&self.sleep_text_area
                            oninput=self.link.callback(|e: InputData| Msg::SleepTextAreaUpdated(e.value))
                            placeholder="how you slept">
                        </textarea>
                        <br/>
                        <button onclick=self.link.callback(|_| Msg::SubmitSleep)>{ "Submit" }</button>

                        <p> { "Records: " } { &self.sleep_entries.len() } </p>
                    </div>

                    <div>
                        <textarea
                            rows=6
                            value=&self.notes_text_area
                            oninput=self.link.callback(|e: InputData| Msg::NotesTextAreaUpdated(e.value))
                            placeholder="notes">
                        </textarea>
                        <br/>
                        <button onclick=self.link.callback(|_| Msg::SubmitNotes)>{ "Submit" }</button>

                        <p> { "Records: " } { &self.notes.len() } </p>
                    </div>
                </div>

                <div id="moodgrid">
                    { self.mood_readings.iter().map(render_mood_bar).collect::<Html>() }
                </div>

                <div id="dategrid">
                    { self.mood_readings.iter().map(render_mood_date).collect::<Html>() }
                </div>
            </div>
        }
    }
}

fn render_mood_bar(r: &MoodReading) -> Html {
    let value = r.value;
    html! {
        <>
            <div class={class_from(value, 3)}></div>
            <div class={class_from(value, 2)}></div>
            <div class={class_from(value, 1)}></div>
            <div class={class_from(value, 0)}></div>
            <div class={class_from(value, -1)}></div>
            <div class={class_from(value, -2)}></div>
            <div class={class_from(value, -3)}></div>
        </>
    }
}

fn render_mood_date(r: &MoodReading) -> Html {
    let dt = Utc.timestamp_millis(r.epoch_millis as i64);
    let date_string = dt.format("%m/%d").to_string();
    html! {
        <>
            <div class="date">{ date_string }</div>
        </>
    }
}

fn class_from(value: i8, position: i8) -> String {
    if position == 0 {
        "neutral".to_string()
    } else if position > 0 && value >= position {
        format!("hot{}", position)
    } else if position < 0 && value <= position {
        format!("cold{}", position.abs())
    } else {
        "nocolor".to_string()
    }
}

const DAYS_TO_DISPLAY: u8 = 14;

fn recent_moods(mrs: Vec<MoodReading>) -> Vec<MoodReading> {
    use chrono::prelude::*;
    let grouped = group_by(mrs, |mr| {
        Utc.timestamp_millis(mr.epoch_millis as i64).date()
    });

    let cutoff = Utc::now().date() - chrono::Duration::days(DAYS_TO_DISPLAY as i64);

    let recent_grouped = grouped.iter().filter(|(date, _)| date > &cutoff);

    let max_in_each = recent_grouped.map(|(_date, mood_readings)| wildest(mood_readings));

    max_in_each
        .map(|at_most_two| match at_most_two {
            HighLowMoods::Nothing => vec![],
            HighLowMoods::One(mr) => vec![mr],
            HighLowMoods::MaxMin(h, l) => vec![h, l],
        })
        .flatten()
        .collect()
}

enum HighLowMoods {
    Nothing,
    One(MoodReading),
    MaxMin(MoodReading, MoodReading),
}

fn wildest(readings: &Vec<MoodReading>) -> HighLowMoods {
    let mut lowest: Option<MoodReading> = None;
    let mut nil: Option<MoodReading> = None;
    let mut highest: Option<MoodReading> = None;
    for mr in readings {
        if mr.value < 0 && mr.value < lowest.map(|l| l.value).unwrap_or(0) {
            lowest = Some(*mr)
        } else if mr.value == 0 && nil.is_none() {
            nil = Some(*mr)
        } else if mr.value > 0 && mr.value > highest.map(|h| h.value).unwrap_or(0) {
            highest = Some(*mr)
        }
    }

    match (lowest, nil, highest) {
        (None, None, None) => HighLowMoods::Nothing,
        (None, Some(mr), None) => HighLowMoods::One(mr),
        (Some(l), _, None) => HighLowMoods::One(l),
        (None, _, Some(h)) => HighLowMoods::One(h),
        (Some(l), _, Some(h)) => HighLowMoods::MaxMin(h, l),
    }
}

fn group_by<I, F, K, T>(xs: I, mut key_fn: F) -> Vec<(K, Vec<T>)>
where
    I: IntoIterator<Item = T>,
    F: FnMut(&T) -> K,
    K: Eq,
{
    let mut groups = Vec::<(K, Vec<T>)>::new();
    for item in xs {
        let key = key_fn(&item);
        let last = groups.last_mut();
        if let Some((_, group)) = last.filter(|(k, _)| k == &key) {
            group.push(item);
        } else {
            groups.push((key, vec![item]));
        }
    }
    groups
}
