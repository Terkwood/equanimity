use crate::*;
use chrono::prelude::{DateTime, FixedOffset};
use group_by::group_by;

const DAYS_TO_DISPLAY: u8 = 14;

pub fn recent(
    mrs: &[MoodReading],
    right_now_ms: u64,
    local_datetime: fn(u64) -> DateTime<FixedOffset>,
) -> Vec<MoodReading> {
    let grouped = group_by(mrs, |mr| local_datetime(mr.epoch_millis).date());

    let cutoff =
        local_datetime(right_now_ms).date() - chrono::Duration::days(DAYS_TO_DISPLAY as i64);

    let recent_grouped = grouped.iter().filter(|(date, _)| date > &cutoff);

    let max_in_each = recent_grouped.map(|(_date, mood_readings)| wildest(mood_readings));

    let mut maybe_more_entries_than_allowed: Vec<MoodReading> = max_in_each
        .map(|at_most_two| match at_most_two {
            HighLowMoods::Nothing => vec![],
            HighLowMoods::One(mr ) => vec![mr],
            HighLowMoods::MaxMin(h, l ) => vec![h, l],
        })
        .flatten()
        .collect();

    maybe_more_entries_than_allowed.sort();

    if maybe_more_entries_than_allowed.len() > DAYS_TO_DISPLAY as usize {
        maybe_more_entries_than_allowed
            .iter()
            .skip(maybe_more_entries_than_allowed.len() - DAYS_TO_DISPLAY as usize)
            .cloned()
            .collect()
    } else {
        maybe_more_entries_than_allowed
    }
}

#[derive(Debug, PartialEq)]
pub enum HighLowMoods {
    Nothing,
    One(MoodReading ),
    MaxMin(MoodReading, MoodReading ),
}


fn wildest(readings: &Vec<&MoodReading>) -> HighLowMoods {
    let mut lowest: Option<MoodReading> = None;
    let mut nil: Option<MoodReading> = None;
    let mut highest: Option<MoodReading> = None;
    for mr in readings {
        if mr.value < 0 && mr.value < lowest.map(|l| l.value).unwrap_or(0) {
            lowest = Some(**mr)
        } else if mr.value == 0 && nil.is_none() {
            nil = Some(**mr)
        } else if mr.value > 0 && mr.value > highest.map(|h| h.value).unwrap_or(0) {
            highest = Some(**mr)
        }
    }

    match (lowest, nil, highest) {
        (None, None, None) => HighLowMoods::Nothing,
        (None, Some(mr), None) => HighLowMoods::One(mr   ),
        (Some(l), _, None) => HighLowMoods::One(l ),
        (None, _, Some(h)) => HighLowMoods::One(h ),
        (Some(l), _, Some(h)) => HighLowMoods::MaxMin(h, l ),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::prelude::*;

    const ONE_DAY_MS: u64 = 86_400_000;

    const LOCAL_OFFSET_SECONDS: i32 = 240;
    fn fake_local_datetime(epoch_millis_utc: u64) -> DateTime<FixedOffset> {
        let offset = FixedOffset::west(LOCAL_OFFSET_SECONDS);
        Utc.timestamp_millis(epoch_millis_utc as i64)
            .with_timezone(&offset)
    }

    #[test]
    fn test_recent_moods_dedup() {
        let right_now = Utc::now().timestamp_millis() as u64;

        let exp00 = MoodReading {
            value: -2,
            epoch_millis: right_now,
        };

        let exp01 = MoodReading {
            value: 3,
            epoch_millis: right_now - ONE_DAY_MS,
        };

        let exp02_a = MoodReading {
            value: 3,
            epoch_millis: right_now - 2 * ONE_DAY_MS,
        };

        let exp02_b = MoodReading {
            value: -3,
            epoch_millis: right_now - 2 * ONE_DAY_MS - 1,
        };

        let exp03_a = MoodReading {
            value: -1,
            epoch_millis: right_now - 3 * ONE_DAY_MS,
        };

        let exp03_b = MoodReading {
            value: 2,
            epoch_millis: right_now - 3 * ONE_DAY_MS - 1,
        };

        let convoluted = vec![
            MoodReading {
                value: 0,
                epoch_millis: right_now,
            },
            MoodReading {
                value: -1,
                epoch_millis: right_now,
            },
            exp00,
            exp01,
            MoodReading {
                value: 2,
                epoch_millis: right_now - ONE_DAY_MS,
            },
            MoodReading {
                value: 1,
                epoch_millis: right_now - ONE_DAY_MS,
            },
            exp02_a,
            exp02_b,
            MoodReading {
                value: 0,
                epoch_millis: right_now - 2 * ONE_DAY_MS,
            },
            exp03_a,
            MoodReading {
                value: 0,
                epoch_millis: right_now - 3 * ONE_DAY_MS,
            },
            exp03_b,
        ];

        let actual = recent(&convoluted, right_now, fake_local_datetime);

        let mut last_timestamp = 0;
        for a in &actual {
            assert!(a.epoch_millis > last_timestamp);
            last_timestamp = a.epoch_millis;
        }

        let mut expected = vec![exp00, exp01, exp02_a, exp02_b, exp03_a, exp03_b];
        expected.reverse();

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_recent_mood_dedup_2() {
        let right_now = Utc::now().timestamp_millis() as u64;

        let mut many_dup_vals = vec![];
        for i in 0..10 {
            many_dup_vals.push(MoodReading {
                value: 0,
                epoch_millis: right_now - i * ONE_DAY_MS,
            });
            many_dup_vals.push(MoodReading {
                value: 0,
                epoch_millis: right_now - i * ONE_DAY_MS,
            });
            many_dup_vals.push(MoodReading {
                value: 0,
                epoch_millis: right_now - i * ONE_DAY_MS,
            });
            many_dup_vals.push(MoodReading {
                value: 0,
                epoch_millis: right_now - i * ONE_DAY_MS,
            });
        }

        let result = recent(&many_dup_vals, right_now, fake_local_datetime);
        assert_eq!(10, result.len());
    }

    const ONE_HOUR_MS: u64 = 3_600_000;

    #[test]
    fn test_wildest_suppresses_zero() {
        let right_now = Utc::now().timestamp_millis() as u64;

        let mut back_and_forth = vec![];

        for i in 0..6 {
            let value = if i % 2 == 0 { 0 } else { -1 };
            let m = MoodReading {
                epoch_millis: right_now - i * ONE_HOUR_MS,
                value,
            };
            back_and_forth.push(m)
        }

        let actual = wildest(&back_and_forth.iter().map(|m| m).collect());

        assert!(match actual {
            HighLowMoods::One(MoodReading {
                value: -1,
                epoch_millis: _,
            }) => true,
            _ => false,
        })
    }

    #[test]
    fn test_wildest_suppresses_zero_2() {
        let readings = vec![
            MoodReading {
                epoch_millis: 1597523124084,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597523122690,
                value: -1,
            },
            MoodReading {
                epoch_millis: 1597493415667,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597493132454,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597491323691,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597485707821,
                value: -1,
            },
            MoodReading {
                epoch_millis: 1597452104644,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597425244763,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597407014195,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597361837123,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597349743059,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597341509304,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597334424046,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597326808955,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597318625023,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597312187831,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597281016460,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597276242288,
                value: 0,
            },
        ];

        let actual = wildest(&readings.iter().map(|r| r).collect());

        assert!(match actual {
            HighLowMoods::One(MoodReading {
                value: -1,
                epoch_millis: _,
            }) => true,
            _ => false,
        })
    }

    #[test]
    fn test_local_timezone_sanity_no_dups() {
        let bunches = vec![
            MoodReading {
                epoch_millis: 1597702278763,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597681202925,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597671886589,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597668627387,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597660254860,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597649650958,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597592618604,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597576524553,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597543842972,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597526972008,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597523124084,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597523122690,
                value: -1,
            },
            MoodReading {
                epoch_millis: 1597493415667,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597493132454,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597491323691,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597485707821,
                value: -1,
            },
            MoodReading {
                epoch_millis: 1597452104644,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597425244763,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597407014195,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597361837123,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597349743059,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597341509304,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597334424046,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597326808955,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597318625023,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597312187831,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597281016460,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597276242288,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597272995216,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597272349945,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597270780125,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597267554233,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597263312498,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597258096295,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597256944336,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597255118798,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597255117475,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597251241611,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597250269535,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597244236242,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597233653458,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597218655745,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597176031749,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597165377041,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597157950814,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597144162773,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597099974869,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597067839254,
                value: 1,
            },
            MoodReading {
                epoch_millis: 1597061104712,
                value: 1,
            },
            MoodReading {
                epoch_millis: 1597049490659,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597041862256,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597015425013,
                value: 0,
            },
            MoodReading {
                epoch_millis: 1597715308686,
                value: 0,
            },
        ];

        let right_now = Utc::now().timestamp_millis() as u64;

        let actual = recent(&bunches, right_now, fake_local_datetime);

        let mut actual_local_dates: Vec<String> = actual
            .iter()
            .map(|mr| {
                let dt = fake_local_datetime(mr.epoch_millis);
                let date_string = dt.format("%m/%d").to_string();
                date_string
            })
            .collect();

        let num_dates_before_dedup = actual_local_dates.len();

        actual_local_dates.sort();
        actual_local_dates.dedup();

        assert_eq!(num_dates_before_dedup, actual_local_dates.len())
    }
}
