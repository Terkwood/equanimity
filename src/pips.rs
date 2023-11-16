use std::collections::HashMap;
use std::cmp::max;
use std::cmp::min;

use crate::*;
use chrono::NaiveDateTime;

const MANIC_CIRCLE: char = '🔴';
const DEPRESSED_CIRCLE: char = '🔵';
const EQUANIMITY_CIRCLE: char = '⚪';
const EMPTY_CIRCLE: char = '⚫';

pub fn circles(moods: &[i8]) -> String {
    let red = brightest_red(moods);
    let blue = deepest_blue(moods);
    let equanimity = had_equanimity(moods);

    let red_circles = format!(
        "{}{}",
        MANIC_CIRCLE.to_string().repeat(red as usize),
        EMPTY_CIRCLE.to_string().repeat(3 - red as usize)
    );
    let blue_circles = format!(
        "{}{}",
        EMPTY_CIRCLE
            .to_string()
            .repeat(3 - (i8::abs(blue) as usize)),
        DEPRESSED_CIRCLE.to_string().repeat(i8::abs(blue) as usize)
    );

    // define a string which shows EQUANIMITY_CIRCLE if equanimity is true, otherwise EMPTY_CIRCLE
    let equanimity_circle = if equanimity {
        EQUANIMITY_CIRCLE
    } else {
        EMPTY_CIRCLE
    };

    format!("{}{}{}", blue_circles, equanimity_circle, red_circles)
}


pub fn day_label(day: &chrono::NaiveDate) -> String {
    day.format("  %a %b %e  ").to_string()
}

pub fn group_by_day(v: &[MoodReading]) -> HashMap<chrono::NaiveDate, Vec<i8>> {
    let mut by_day: HashMap<chrono::NaiveDate, Vec<i8>> = HashMap::new();

    for mood in v {
        if let Some(date) =
            NaiveDateTime::from_timestamp_millis(mood.epoch_millis as i64).map(|t| t.date())
        {
            let list = by_day.entry(date).or_default();
            list.push(mood.value);

            list.dedup();

            list.sort();
        }
    }

    by_day
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
    fn test_group_by_date() {
        let mrs = vec![
            MoodReading { epoch_millis: 0, value: 0 },
            MoodReading { epoch_millis: 999999999, value: 0 }
        ];

        let by_day = group_by_day(&mrs);
        assert_eq!(by_day.len(), 2);
    }

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
