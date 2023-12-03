use crate::*;
use chrono::NaiveDate;
use js_sys::Object;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

const MANIC_CIRCLE: char = '🔴';
const DEPRESSED_CIRCLE: char = '🔵';
const EQUANIMITY_CIRCLE: char = '⚪';
const EMPTY_CIRCLE: char = '⚫';

pub fn circles(moods: &[i8]) -> String {
    let red = brightest_red(moods);
    let blue = deepest_blue(moods);
    let equanimity = had_equanimity(moods);

    // define a string which shows EQUANIMITY_CIRCLE if equanimity is true, otherwise EMPTY_CIRCLE
    let equanimity_circle = if equanimity {
        EQUANIMITY_CIRCLE
    } else {
        EMPTY_CIRCLE
    };

    format!(
        "{}{}{}",
        blue_circles(blue),
        equanimity_circle,
        red_circles(red)
    )
}

pub fn red_circles(red: i8) -> String {
    format!(
        "{}{}",
        MANIC_CIRCLE.to_string().repeat(red as usize),
        EMPTY_CIRCLE.to_string().repeat(3 - red as usize)
    )
}
pub fn blue_circles(blue: i8) -> String {
    format!(
        "{}{}",
        EMPTY_CIRCLE
            .to_string()
            .repeat(3 - (i8::abs(blue) as usize)),
        DEPRESSED_CIRCLE.to_string().repeat(i8::abs(blue) as usize)
    )
}

pub fn blank_label() -> String {
    [NBSP; 7].iter().collect::<String>()
}

pub fn group_by_day(v: &[MoodReading]) -> Vec<(String, Vec<i8>)> {
    let mut by_day: HashMap<String, Vec<i8>> = HashMap::new();

    for mood in v {
        let date_not_uniform_length =
            js_sys::Date::new(&JsValue::from_f64(mood.epoch_millis as f64))
                .to_locale_date_string("en-US", &Object::new())
                .as_string()
                .unwrap();
        let nd = NaiveDate::parse_from_str(&date_not_uniform_length, "%m/%d/%Y").unwrap();
        let date = nd.format("%Y-%m-%d").to_string();
        {
            let list = by_day.entry(date).or_default();
            list.push(mood.value);

            list.dedup();

            list.sort();
        }
    }

    let mut output = by_day.into_iter().collect::<Vec<_>>();
    output.sort();
    output.reverse();
    output
}

fn deepest_blue(moods: &[i8]) -> i8 {
    let smallest = moods.iter().reduce(|a, b| min(a, b));
    if let Some(sm) = smallest {
        if *sm < 1 {
            *sm
        } else {
            0
        }
    } else {
        0
    }
}

fn brightest_red(moods: &[i8]) -> i8 {
    let largest = moods.iter().reduce(|a, b| max(a, b));
    if let Some(l) = largest {
        if *l > -1 {
            *l
        } else {
            0
        }
    } else {
        0
    }
}

fn had_equanimity(moods: &[i8]) -> bool {
    moods
        .iter()
        .find(|mood| **mood == 0)
        .map(|_| true)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_one() {
        let mr = MoodReading {
            epoch_millis: 0,
            value: 0,
        };
        let s = circles(&[mr.value]);
        assert_eq!(s, "⚫⚫⚫⚪⚫⚫⚫");
    }
    #[test]
    fn test_draw_one_no_eq() {
        let mr = MoodReading {
            epoch_millis: 0,
            value: 1,
        };
        let s = circles(&[mr.value]);
        assert_eq!(s, "⚫⚫⚫⚫🔴⚫⚫");
    }

    #[test]
    fn test_draw_one_with_eq() {
        let s = circles(&[1, 0]);
        assert_eq!(s, "⚫⚫⚫⚪🔴⚫⚫");
    }

    #[test]
    fn test_draw_multi() {
        let s = circles(&[1, 3, -2, -1, 0]);
        assert_eq!(s, "⚫🔵🔵⚪🔴🔴🔴");
    }
}
