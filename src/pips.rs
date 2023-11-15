use std::collections::HashMap;

use crate::{moods::HighLowMoods, *};

// pub fn draw(v: &[MoodReading]) -> String {
//     let by_day = group_by_day(v);

//     let mut s = String::new();
//     for mr in by_day {
//         match mr.0  {
//             HighLowMoods::Nothing => {},
//             HighLowMoods::One(mr, eq) => {s.
//                 push_str(&draw_one(&mr, eq));},
//             HighLowMoods::MaxMin(max, min,eq) => {
//                 s.push_str(&draw_one(&max, eq));
//                 s.push_str(&draw_one(&min, eq));
//             }

//         }

//     }

//     s
// }

// /// but it will return two readings on a given day
// /// if it needs to print both manic and depressive
// fn group_by_day(v: &[MoodReading]) -> Vec<(HighLowMoods, WithEquanimity)> {
//     let mut by_day: HashMap<chrono::NaiveDate, (MoodReading, WithEquanimity)> = HashMap::new();

//     for mr in v {
//         let time = chrono::NaiveDateTime::from_timestamp_millis(mr.epoch_millis as i64);
//         let date = time.map(|t | t.date());
//         if let Some(d) = date {
//             let entry = by_day.entry(d).or_insert(
//                 (
//                     mr.clone(),
//                 (mr.value == 0).into()
//             ));
//             let new_eq = if mr.value == 0  {
//                WithEquanimity::Yes
//             } else { WithEquanimity::No};
//         }
//     }
//     panic!("group by day");
// }

// fn draw_one(mr: &MoodReading, with_equanimity: WithEquanimity) -> String {
//    let s = format!("{}{}{}", depressive_pips(mr), equanimity_pip(mr, with_equanimity == WithEquanimity::Yes), manic_pips(mr) );
//    s
// }

// fn equanimity_pip(mr: &MoodReading, with_equanimity: bool) -> String {
//     if with_equanimity || mr.value == 0 {
//         "⚪".to_string()
//     } else {
//         "⚫".to_string()
//     }
// }

// fn manic_pips(mr: &MoodReading) -> String {
//     let mut s = String::new();
//     for _ in 0..mr.value.abs() {
//         s.push_str("🔴");
//     }
//     for _ in mr.value.abs()..3 {
//         s.push_str("⚫");
//     }
//     let r = s.chars().rev().collect();
//     r
// }
// fn depressive_pips (mr: &MoodReading) -> String {
//     let mut s = String::new();
//     for _ in 0..mr.value.abs() {
//         s.push_str("🔵");
//     }
//     for _ in mr.value.abs()..3 {
//         s.push_str("⚫");
//     }
//     s
// }

fn group_by_day(v: &[MoodReading]) -> HashMap<chrono::NaiveDate, Vec<MoodReading>> {


    // // group mood readings by day
    // // create a map from day (string) to list of mood readings (number[])
    // const byDay: Map<string, number[]> = new Map<string, number[]>();

    // // for each mood reading
    // for (const mood of sample.mood_readings) {
    //     // convert epoch_millis to a date
    //     const d = new Date(mood.epoch_millis);
    //     // get the day (yyyy-mm-dd)
    //     const day = d.toISOString().slice(0, 10);
    //     // add this mood reading to the list of mood readings for this day
    //     // if this is the first mood reading for this day, create a new list
    //     const list = byDay.get(day) ?? [];
    //     list.push(mood.value);

    //     // deduplicate entries in the list
    //     const deduped = [...new Set(list)];
    //     // sort the list
    //     deduped.sort((a, b) => a - b);

    //     // store the list in the map
    //     byDay.set(day, deduped);
    // }
    // // store the keys of byDay, sorted, in a list
    // const days = [...byDay.keys()].sort();

    unimplemented!()
}







const MANIC_CIRCLE: char = '🔴';
const DEPRESSED_CIRCLE: char = '🔵';
const EQUANIMITY_CIRCLE: char = '⚪';
const EMPTY_CIRCLE: char = '⚫';

 fn circles(moods: &[i8]) -> String {
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
        EMPTY_CIRCLE.to_string().repeat(3 - (blue as usize)),
        DEPRESSED_CIRCLE.to_string().repeat(blue as usize)
    );

    // define a string which shows EQUANIMITY_CIRCLE if equanimity is true, otherwise EMPTY_CIRCLE
    let equanimity_circle = if equanimity {
        EQUANIMITY_CIRCLE
    } else {
        EMPTY_CIRCLE
    };

    format!("{}{}{}", blue_circles, equanimity_circle, red_circles)
}

use std::cmp::max;
use std::cmp::min;

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

    // #[test]
    // fn test_draw_one() {
    //     let mr = MoodReading {
    //         epoch_millis: 0,
    //         value: 0,
    //     };
    //     let s = draw_one(&mr, WithEquanimity::Yes);
    //     assert_eq!(s, "⚫⚫⚫⚪⚫⚫⚫");
    // }
    // #[test]
    // fn test_draw_one_no_eq() {
    //     let mr = MoodReading {
    //         epoch_millis: 0,
    //         value: 1,
    //     };
    //     let s = draw_one(&mr, WithEquanimity::No);
    //     assert_eq!(s, "⚫⚫⚫⚫🔴⚫⚫");
    // }
    // #[test]
    // fn test_draw_one_1() {
    //     let mr = MoodReading {
    //         epoch_millis: 0,
    //         value: 1,
    //     };
    //     let s = draw_one(&mr, WithEquanimity::Yes);
    //     assert_eq!(s, "⚫⚫⚫⚪🔴⚫⚫");
    // }
    // #[test]
    // fn test_draw_one_2() {
    //     let mr = MoodReading {
    //         epoch_millis: 0,
    //         value: 2,
    //     };
    //     let s = draw_one(&mr, WithEquanimity::No);
    //     assert_eq!(s, "⚫⚫⚫⚫🔴🔴⚫");
    // }
    // #[test]
    // fn test_draw_one_2_a() {
    //     let mr = MoodReading {
    //         epoch_millis: 0,
    //         value: 2,
    //     };
    //     let s = draw_one(&mr, WithEquanimity::Yes);
    //     assert_eq!(s, "⚫⚫⚫⚪🔴🔴⚫");
    // }

    // #[test]
    // fn test_depressive_draw_one_no_eq() {
    //     let mr = MoodReading {
    //         epoch_millis: 0,
    //         value: -1,
    //     };
    //     let s = draw_one(&mr, WithEquanimity::No);
    //     assert_eq!(s, "⚫⚫🔵⚫⚫⚫⚫");
    // }
    // #[test]
    // fn test_depressive_draw_one_1() {
    //     let mr = MoodReading {
    //         epoch_millis: 0,
    //         value: -1,
    //     };
    //     let s = draw_one(&mr, WithEquanimity::Yes);
    //     assert_eq!(s, "⚫⚫🔵⚪⚫⚫⚫");
    // }
    // #[test]
    // fn test_depressive_draw_one_2() {
    //     let mr = MoodReading {
    //         epoch_millis: 0,
    //         value: -2,
    //     };
    //     let s = draw_one(&mr, WithEquanimity::No);
    //     assert_eq!(s, "⚫🔵🔵⚫⚫⚫⚫");
    // }
    // #[test]
    // fn test_depressive_draw_one_2_a() {
    //     let mr = MoodReading {
    //         epoch_millis: 0,
    //         value: -2,
    //     };
    //     let s = draw_one(&mr, WithEquanimity::Yes);
    //     assert_eq!(s, "⚫🔵🔵⚪⚫⚫⚫");
    // }
}
