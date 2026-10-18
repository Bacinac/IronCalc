#![allow(clippy::unwrap_used)]

use crate::{
    formatter::format::format_number,
    locale::{get_locale, Locale},
};

fn get_default_locale() -> &'static Locale {
    get_locale("en").unwrap()
}

#[test]
fn simple_test() {
    let locale = get_default_locale();
    let format = "h:mm AM/PM";
    let value = 16.001_423_611_111_11; // =1/86400 => 12:02 AM
    let formatted = format_number(value, format, locale);
    assert_eq!(formatted.text, "12:02 AM");
}

#[test]
fn padded_vs_unpadded() {
    let locale = get_default_locale();
    let padded_format = "hh:mm:ss AM/PM";
    let unpadded_format = "h:m:s AM/PM";
    let value = 0.25351851851851853; // => 6:05:04 AM (21904/(24*60*60)) where 21904 = 6 * 3600 + 5*60 + 4
    let formatted = format_number(value, padded_format, locale);
    assert_eq!(formatted.text, "06:05:04 AM");

    let formatted = format_number(value, unpadded_format, locale);
    assert_eq!(formatted.text, "6:5:4 AM");
}

#[test]
fn elapsed_hour() {
    let locale = get_default_locale();
    let format = "[hh]:mm:ss";
    // 1.5 days = 36 hours
    let value = 1.5;
    let formatted = format_number(value, format, locale);
    assert_eq!(formatted.text, "36:00:00");
}

#[test]
fn elapsed_minute() {
    let locale = get_default_locale();
    let format = "[mm]:ss";
    // 2 days = 2880 minutes
    let value = 2.0;
    let formatted = format_number(value, format, locale);
    assert_eq!(formatted.text, "2880:00");
}

#[test]
fn elapsed_second() {
    let locale = get_default_locale();
    let format = "[ss]";
    // 0.5 days = 43200 seconds
    let value = 0.5;
    let formatted = format_number(value, format, locale);
    assert_eq!(formatted.text, "43200");
}

#[test]
fn elapsed_hour_padded() {
    let locale = get_default_locale();
    let format = "[hh]:mm:ss";
    // 0.1 days = 2.4 hours
    let value = 0.1;
    let formatted = format_number(value, format, locale);
    assert_eq!(formatted.text, "02:24:00");
}

#[test]
fn rounded_to_the_second_before_it_is_split() {
    let locale = get_default_locale();
    // =DATE(2026,10,18)+TIME(13,45,0) lands just below 13:45 in binary.
    let value = 46313.0 + (13.0 * 3600.0 + 45.0 * 60.0) / 86400.0;
    assert_eq!(format_number(value, "hh:mm", locale).text, "13:45");
    assert_eq!(format_number(value, "hh:mm:ss", locale).text, "13:45:00");
    assert_eq!(format_number(value, "[h]:mm", locale).text, "1111525:45");
    let late = 46313.0 + 86399.9999 / 86400.0;
    assert_eq!(
        format_number(late, "m/d/yyyy hh:mm:ss", locale).text,
        "10/19/2026 00:00:00"
    );
    let third = 0.1 + 0.2;
    assert_eq!(format_number(third, "[mm]:ss", locale).text, "432:00");
}
