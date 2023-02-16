use anyhow::Context;
use chrono::{DateTime, Utc};

pub fn render_datetime_and_info(datetime: DateTime<Utc>) -> String {
    let days_passed = (Utc::now() - datetime).num_days();
    let at_least_one_day_passed = days_passed > 0;

    let extra_info = at_least_one_day_passed
        .then(|| {
            let multiple_days_passed = days_passed > 1;
            let day_word = if multiple_days_passed { "days" } else { "day" };
            format!(" ({days_passed} {day_word} ago)")
        })
        .unwrap_or_default();
    format!("{}{extra_info}", datetime.format("%d.%m.%Y"),)
}

pub fn ask_datetime(prompt: &str) -> anyhow::Result<DateTime<Utc>> {
    let naive_date = inquire::DateSelect::new(prompt).prompt()?;
    let naive_datetime = naive_date
        // just use some random time
        .and_hms_opt(12, 0, 0)
        .context("couldn't convert NaiveDate to NaiveDateTime")?;

    Ok(DateTime::from_utc(naive_datetime, Utc))
}
