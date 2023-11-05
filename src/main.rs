// holocene a holcene (or human era) date print out.
// References:
// Core utils date util unit tests: https://github.com/coreutils/coreutils/blob/56e9acb2927874770363462cc6f5d2ee7d2b4c6d/tests/date/date.pl#L101
// Core utils date util: https://github.com/coreutils/coreutils/blob/master/src/date.c

/*
TODO: Create a closer parity with the linux date command.
https://linux.die.net/man/1/date
-----------
Name: date - print or set the system date and time

Synopsis:
date [OPTION]... [+FORMAT]
date [-u|--utc|--universal] [MMDDhhmm[[CC]YY][.ss]]

Description: Display the current time in the given FORMAT, or set the system date.
-----------

-d, --date=STRING
display time described by STRING, not 'now'

-f, --file=DATEFILE
like --date once for each line of DATEFILE

-r, --reference=FILE
display the last modification time of FILE

-R, --rfc-2822
output date and time in RFC 2822 format. Example: Mon, 07 Aug 2006 12:34:56 -0600

--rfc-3339=TIMESPEC
output date and time in RFC 3339 format. TIMESPEC='date', 'seconds', or 'ns' for date and time to the indicated precision. Date and time components are separated by a single space: 2006-08-07 12:34:56-06:00

-s, --set=STRING
set time described by STRING

-u, --utc, --universal
print or set Coordinated Universal Time

--help
display this help and exit

--version
output version information and exit

-----------
FORMAT
controls the output.
-----------
Interpreted sequences are:

By default, date pads numeric fields with zeroes. The following optional flags may follow '%':

-
(hyphen) do not pad the field

_
(underscore) pad with spaces
    (zero) pad with zeros

^
use upper case if possible

#
use opposite case if possible

After any flags comes an optional field width, as a decimal number; then an optional modifier,
which is either E to use the locale's alternate representations if available, or O to use the
locale's alternate numeric symbols if available.

-----------
Date String
-----------
The --date=STRING is a mostly free format human readable date string such as
"Sun, 29 Feb 2004 16:21:42 -0800" or "2004-02-29 16:21:42" or even "next Thursday".
A date string may contain items indicating calendar date, time of day, time zone, day of week,
relative time, relative date, and numbers. An empty string indicates the beginning of the day.
The date string format is more complex than is easily documented here but is
fully described in the info documentation.

-----------
Environment
-----------
TZ
Specifies the timezone, unless overridden by command line parameters. If neither is specified, the setting from /etc/localtime is used.
*/

// holocene a holcene (or human era) date print out application analogous to the linux date command.
// Aims to be a very close clone of the linux date command without the setting of the system date.
// Also designed to convert convert BCE dates to Holocene dates.

use std::collections::HashMap;
use std::fmt::Display;

use chrono::{DateTime, LocalResult};
use chrono::offset::TimeZone;
use chrono::prelude::*;
use clap::{arg, Command, Parser};
use regex::Regex;

fn main() {
    let mut matches;

    let mut cmd = Command::new("holocene")
        .version("0.1.0")
        .author("Collin Dutrow")
        .about("Holocene date command")
        .arg(arg!([format] ... "Format string").trailing_var_arg(true))
        .arg(arg!(-d --date [STRING] "Date string"));

    if cfg!(debug_assertions) {
        matches = cmd.get_matches_from(vec!["testapp", "-d", "yesterday"]);
        println!("Debug mode. Launch parameters are locked and pre-set.")
    } else {
        matches = cmd.get_matches();
    }

    let date = matches.try_get_one::<String>("date").unwrap();
    let mut formatter: Option<String> = None;

    let trailing = match matches.try_get_many::<String>("format") {
        Ok(Some(values)) => values.cloned().collect::<Vec<String>>(),
        Ok(None) => {
            // Handle cases when the "format" argument is not provided
            Vec::new() // An empty Vec if no values are provided
        },
        Err(e) => {
            // Handle error cases
            Vec::new() // Or handle the error as needed
        }
    };

    // If there is a trailing string then it must be a format string if it starts with a + sign.
    if !trailing.is_empty() && trailing.first().unwrap().starts_with("+") {
        let format = str_rm_char_first(trailing.first().unwrap()).to_string();
        formatter = Some(format);
    }

    // Check if -d or --date is set.
    if matches.try_contains_id("date").unwrap() {
        if let Some(ref String) = formatter {
            // If there is a date string and a formatter string, print the date string using the formatter string.
            println!("{}", parse_date_string::<Utc>(date.unwrap(), Some(&formatter.unwrap())));
        } else {
            // If there is a date string but no formatter string, print the date string using the default format.
            println!("{}", parse_date_string::<Utc>(date.unwrap(), None));
        }
    } else if let Some(ref String) = formatter {
        // If there is no date string but there is a formatter string, print the current date using the formatter string.
        println!("{}", parse_date_string::<Utc>("now", Some(&formatter.unwrap())));
    } else {
        // If there is no date string or formatter string, print the current date using the default format.
        println!("{}", parse_date_string::<Utc>("now", None));
    }

    assert_eq!(parse_date_string::<Utc>("12/10/1995", None), "Sun 12 10 00:00:00 AM UTC 11995");
    assert_eq!(parse_date_string::<Utc>("1995/12/10", None), "Sun 12 10 00:00:00 AM UTC 11995");
    assert_eq!(parse_date_string::<Utc>("12:01:57 12/10/1995", None), "Sun 12 10 12:01:57 PM UTC 11995");
    assert_eq!(parse_date_string::<Utc>("12/10/1995 12:01:57", None), "Sun 12 10 12:01:57 PM UTC 11995");
    assert_eq!(parse_date_string::<Utc>("+saz 12/10/1995 12:01:57 asdfz", None), "Sun 12 10 12:01:57 PM UTC 11995");
    assert_eq!(parse_date_string::<Utc>("+saz 12/10/9999 12:01:57 BCE asdfz", None), "Fri 12 10 12:01:57 PM UTC 2");
    assert_eq!(parse_date_string::<Utc>("+saz 12/10/9999 12:01:57 BCE asdfz", Some("%m-%d-%Y %H:%M:%S %z %Z %E")), "12-10-2 12:01:57 +0000 UTC HE");
    assert_eq!(parse_date_string::<Utc>("+saz 12/10/0436 12:01:57 BCE asdfz", None), "Wed 12 10 12:01:57 PM UTC 9565");
    assert_eq!(parse_date_string::<Utc>("+saz 12/10/0436 12:01:57 BCE asdfz", Some("%m-%d-%Y %H:%M:%S %z %Z %E")), "12-10-9565 12:01:57 +0000 UTC HE");
}

///
///
/// # Arguments
///
/// * `dt`: DateTime<T> - A chrono DateTime object
///
/// returns: String - A string containing the Holocene year
///
/// # Examples
///
/// ```
/// println!("Current Holocene Year: {}", holocene_year(Local::now()));
/// ```
fn holocene_year<T: TimeZone>(dt: DateTime<T>) -> String {
    let holocene_year = dt.year() + 10_000 + 1;
    return format!("{}", holocene_year);
}

/// Attempts to parse a flexible expression of a date string into a chrono DateTime object.
/// Designed to be comparable to the UNIX date command.
///
/// # Arguments
///
/// * `input`:
///
/// returns: Result<DateTime<Utc>, &str>
///
/// # Examples
///
/// ```
///
/// ```
fn parse_date_string<T>(input: &str, formatter: Option<&str>) -> String {
    // Extract date from strings like "YYYY/MM/DD HH:MM:SS" and "MM/DD/YYYY HH:MM:SS" where the month and or time are optional.
    // If the string contains BCE or BC then subtract the current year from the year in the string.
    // If the string contains CE or AD then just remove the CE or AD from the string.
    // Figure out the date from strings like +/-1 second(s), minute(s), day(s), week(s), month(s), year(s) E.G. "+1 day" and "-1 year"
    // Also figure out the date from strings like last/next as +1 <measurement> and -1 <measurement> respectively.
    // Also figure out the date from strings like last/next <day of week> E.G. "last Monday" and "next Monday"
    // Also figure out the date from strings like "mm/dd/yy" and "mm-dd-yy"
    // Ignore extra spaces in the string except for 'next/last' strings.

    // Trim the input string.
    let input = input.trim();

    // TODO: implement proper support for timezones.
    // TODO: add support for "ago" E.G. "1 day ago"

    // Regexes for parsing the date segments from the input string.
    let mut re_mdy = Regex::new(r"(?:(?P<month>\d{1,2})/(?P<day>\d{1,2})/(?P<year>\d{4}))").unwrap();
    let mut re_ymd = Regex::new(r"(?:(?P<year>\d{4})/(?P<month>\d{1,2})/(?P<day>\d{1,2}))").unwrap();
    let re_hms = Regex::new(r"(?:(?P<hour>\d{1,2}):(?P<minute>\d{1,2}):(?P<second>\d{1,2}))").unwrap();
    let re_relative = Regex::new(r"^\s*(?P<sign>[+-])(?P<value>\d+)\s+(?P<unit>second|minute|hour|day|week|month|year)s?\s*$").unwrap();
    let re_relative_word = Regex::new(r"^\s*(?P<sign>next|last)\s+(?P<unit>second|minute|hour|day|week|month|year)s?\s*$").unwrap();
    let re_timezone = Regex::new(r"^(?P<timezone>[A-Z]{3,5})$").unwrap();
    let re_year_notation = Regex::new(r"(?i)(BCE|BC)").unwrap();

    let mut input_date: LocalResult<DateTime<Utc>> = chrono::LocalResult::Single(Utc::now());

    // TODO: Fix matches to show the correct time (h:m:s).
    // Do a switch case for the input string and handle words like "now", "today", "yesterday", "tomorrow", "fortnight"
    match input {
        ("now" | "today") => {
            input_date = chrono::LocalResult::Single(Utc::now());
        },
        "yesterday" => {
            input_date = chrono::LocalResult::Single(Utc::now() + chrono::Duration::days(-1));
        },
        "tomorrow" => {
            input_date = chrono::LocalResult::Single(Utc::now() + chrono::Duration::days(1));
        },
        "fortnight" => {
            // Set the date to 14 days in the future.
            input_date = chrono::LocalResult::Single(Utc::now() + chrono::Duration::days(14));
        },
        _ => {}
    }

    // TODO: Implement support for +/-1 <unit> E.G. "+1 day" and "-1 year"

    let mut date_found = false;
    let mut time_found = false;
    let mut before_ce = false;

    let mut year: i32 = 0;
    let mut month: u32 = 0;
    let mut day: u32 = 0;
    let mut hour: u32 = 0;
    let mut minute: u32 = 0;
    let mut second: u32 = 0;

    let date_regexes = [re_mdy, re_ymd];

    if let Some(caps) = re_year_notation.captures(input) {
        before_ce = true;
    }

    // Extract date segments from the input string.
    for re in date_regexes.iter() {
        if let Some(caps) = re.captures(input) {
            date_found = true;
            year = caps.name("year").unwrap().as_str().parse::<i32>().unwrap();
            month = caps.name("month").unwrap().as_str().parse::<u32>().unwrap();
            day = caps.name("day").unwrap().as_str().parse::<u32>().unwrap();

            // Break after the first successful match.
            break;
        }
    }

    // Extract time segments from the input string.
    if let Some(caps) = re_hms.captures(input) {
        time_found = true;
        hour = caps.name("hour").unwrap().as_str().parse::<u32>().unwrap();
        minute = caps.name("minute").unwrap().as_str().parse::<u32>().unwrap();
        second = caps.name("second").unwrap().as_str().parse::<u32>().unwrap();
    }

    if !date_found && !time_found {
        Err::<T, &str>("No date or time found.");
    } else {
        input_date = Utc.with_ymd_and_hms(year, month, day, hour, minute, second);
    }

    // Convert the year to the Holocene year.
    let mut holocene_year = input_date.unwrap().year() + 10_000;
    let gregorain_year = Local::now().year();

    // If the year is BCE. So convert it to a positive number,
    // then add the current year to it, and then subtract that from the current year + 10,000.
    if before_ce {
        // We need to subtract 1 from the year because there is no year 0 BCE.
        holocene_year = gregorain_year + 10_000 - (gregorain_year + year - 1);
    }

    // If the format string is set, then format the date string and return early.
    if let Some(format_string) = formatter {
        let formatted_date = format_date(format_string, input_date.unwrap());

        // Find and replace the year in the formatted date with the Holocene year.
        // The specifics of this step depend on the format of your date string.
        let holocene_formatted_date = formatted_date.replace(
            &input_date.unwrap().format("%Y").to_string(),
            &holocene_year.to_string(),
        );

        return holocene_formatted_date;
    }

    // Create a new string <WDay> Month Day HH:MM:SS <PM> <PST> YYYY
    let mut date_string = format!("{} {} {} {:02}:{:02}:{:02} {} {} {}",
                                  input_date.unwrap().weekday(),
                                  input_date.unwrap().month(),
                                  input_date.unwrap().day(),
                                  input_date.unwrap().hour(),
                                  input_date.unwrap().minute(),
                                  input_date.unwrap().second(),
                                  input_date.unwrap().format("%p"),
                                  input_date.unwrap().format("%Z"),
                                  holocene_year);

    date_string
}

/// Formats a chrono DateTime object using a custom format.
/// Performs custom formatting and then formats the date using the chrono format function.
///
/// # Arguments
///
/// * `format`: &str - A string containing the desired format template.
/// * `date_time`: DateTime<T> - A chrono DateTime object.
///
/// returns: String - A string containing the formatted date.
///
/// # Examples
///
/// ```
///
/// ```
fn format_date<T: chrono::TimeZone>(format: &str, date_time: DateTime<T>) -> String
    where
        T: TimeZone,
        T::Offset: Display,
{
    let mut custom_format = String::new();
    let mut special_formats: HashMap<&str, Box<dyn Fn(DateTime<T>) -> String>> = HashMap::new();

    // Unix date string formatters are are virtually the same as
    // chrono formatters. https://docs.rs/chrono/latest/chrono/format/strftime/index.html

    special_formats.insert("%N", Box::new(|dt| format!("{:09}", dt.timestamp_subsec_nanos()))); // Nanoseconds
    // New specifiers not in the date command
    special_formats.insert("%E", Box::new(|_| "HE".to_string())); // Custom specifier for "HE"

    let mut chars = format.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            if let Some(next_char) = chars.peek() {
                if *next_char == '%' {
                    // Handle the literal '%' character
                    custom_format.push('%');
                    chars.next(); // Skip the next '%' to prevent double insertion
                } else {
                    // Handle format specifiers
                    let mut specifier = "%".to_string();
                    specifier.push(*next_char);
                    if let Some(special_format) = special_formats.get(&specifier[..]) {
                        custom_format.push_str(&special_format(date_time.clone()));
                        chars.next(); // Skip next char as it is part of format specifier
                    } else {
                        // If the specifier is not recognized, keep it as is
                        custom_format.push('%');
                        custom_format.push(*next_char);
                        chars.next(); // Skip next char as it has been manually added
                    }
                }
            } else {
                // '%' is the last character in the string
                custom_format.push('%');
            }
        } else {
            custom_format.push(c);
        }
    }

    // TODO: Handle issues with % formatters that return a year, as those will need to be converted to holocene years. Keep in mind that some non-0 padded years may be tricky to replace.

    date_time.format(&custom_format).to_string()
}

/// Removes the first character from a string slice.
///
/// # Arguments
///
/// * `value`: &str - A string slice
///
/// returns: &str - The value string slice with the first character removed.
///
/// # Examples
///
/// ```
///
/// ```
fn str_rm_char_first(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}