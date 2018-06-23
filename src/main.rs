extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate chrono_tz;
use chrono::Duration;

#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
struct Initiator {
    email: String,
    name: String,
    notify: bool,
    #[serde(rename="timeZone")]
    time_zone: String,
}

#[derive(Serialize, Deserialize)]
struct Location {
    category: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct TimeOption {
  allday: bool,
  end: i64,
  start: i64,
}

#[derive(Serialize, Deserialize)]
struct DoodleRequest {
    comments: Vec<String>,
    description: String,
    initiator: Initiator,
    locale: String,
    location: Location,
    options: Vec<TimeOption>,
    participants: Vec<String>,
    #[serde(rename="timeZone")]
    time_zone: bool,
    title: String,
    #[serde(rename="type")]
    form_type: String,
}

fn main() {
    let start_time = chrono_tz::America::New_York.ymd(2018, 6, 24).and_hms(0, 0, 0);
    let end_time = chrono_tz::America::New_York.ymd(2018, 7, 2).and_hms(0, 0, 0);
    let duration_between_dates = end_time.signed_duration_since(start_time);
    let intervals = Duration::minutes(30);
    let practice_time = Duration::hours(2);
    let number_of_options = duration_between_dates.num_minutes() / intervals.num_minutes();
    let options: Vec<TimeOption> = (0..number_of_options).map(|i| {
        let option_start = start_time + intervals * (i as i32);
        return TimeOption {
            allday: false,
            start: option_start.timestamp() * 1000,
            end: (option_start + practice_time).timestamp() * 1000
        }
    }).collect();

    let request = DoodleRequest {
        comments: vec![],
        description: "".to_string(),
        initiator: Initiator {
            email: "pocketcookies2@gmail.com".to_string(),
            name: "Pocketcookies".to_string(),
            notify: true,
            time_zone: "America/New_York".to_string(),
        },
        location: Location {
            category: "ONLINE".to_string(),
            name: "Online".to_string(),
        },
        participants: vec![],
        time_zone: true,
        title: "Team Hotpocket Meeting Schedule".to_string(),
        form_type: "DATE".to_string(),
        locale: "en_US".to_string(),
        options: options
    };

    let output: String = serde_json::to_string(&request).unwrap();
    println!("{}", output);
}
