use std::cmp::Ordering;
use std::ops::{Add, Sub};
use std::fmt;

use time;

/// Represents the components of a moment in time in Persian Calendar.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Time {
    pub old_tm: time::Tm,

    pub year: i32,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string("YYYY-MM-DD HH:mm:ss"))
    }
}

// impl Add<time::Duration> for Tm {
//     type Output = Tm;

//     // FIXME: The timezone of `self` is different from resulting time
//     fn add(self, other: time::Duration) -> Tm {
//         at_utc(self.to_timespec() + other)
//     }
// }

// impl Sub<time::Duration> for Tm {
//     type Output = Tm;

//     // FIXME: The timezone of `self` is different from resulting time
//     fn sub(self, other: time::Duration) -> Tm {
//         at_utc(self.to_timespec() - other)
//     }
// }

// impl Sub<Tm> for Tm {
//     type Output = time::Duration;

//     fn sub(self, other: Tm) -> time::Duration {
//         self.to_timespec() - other.to_timespec()
//     }
// }

// impl Sub<time::Tm> for Tm {
//     type Output = time::Duration;

//     fn sub(self, other: time::Tm) -> time::Duration {
//         self.to_timespec() - other.to_timespec()
//     }
// }

// impl PartialOrd for Tm {
//     fn partial_cmp(&self, other: &Tm) -> Option<Ordering> {
//         self.to_timespec().partial_cmp(&other.to_timespec())
//     }
// }

// impl Ord for Tm {
//     fn cmp(&self, other: &Tm) -> Ordering {
//         self.to_timespec().cmp(&other.to_timespec())
//     }
// }

impl Time {
    pub fn new(tm:time::Tm) -> Time {
        let mut rs = Time {
            old_tm : tm,
            year : 0
        };

        rs.year = tm.tm_year + 1900;

        return rs;
    }

    pub fn to_string<'a>(&'a self, format: &'a str) -> String {
        format
            .replace("YYYY", &self.year.to_string())
            .replace("YY", &self.year.to_string()[2..])

            .replace("MMM", match self.old_tm.tm_mon {
                                0 => "Jan",
                                1 => "Feb",
                                2 => "Mar",
                                3 => "Apil",
                                4 => "May",
                                5 => "Jun",
                                6 => "July",
                                7 => "Aust",
                                8 => "Sep",
                                9 => "Oct",
                                10 => "Nov",
                                11 => "Dec",
                                _ => panic!("invalid month value of {}", self.old_tm.tm_mon),
                            })
            .replace("MM", &format!("{:02}", self.old_tm.tm_mon + 1))
            .replace("M", &format!("{}", self.old_tm.tm_mon + 1))

            .replace("DDD", &format!("{}", self.old_tm.tm_yday + 1))
            .replace("DD", &format!("{:02}", self.old_tm.tm_mday))
            .replace("D", &self.old_tm.tm_mday.to_string())
            
            .replace("ddd", match self.old_tm.tm_wday {
                              1 => "Mon",
                              2 => "Tus",
                              3 => "Wed",
                              4 => "Thus",
                              5 => "Fri",
                              6 => "Sat",
                              0 => "Sun",
                              _ => panic!("invalid weekday value of {}", self.old_tm.tm_wday),
                          })
            .replace("d", &self.old_tm.tm_wday.to_string())

            .replace("A", if self.old_tm.tm_hour < 12 {
                              "AM"
                          } else {
                              "PM"
                          })
            .replace("a", if self.old_tm.tm_hour < 12 {
                              "am"
                          } else {
                              "pm"
                          })
            .replace("HH", &format!("{:02}", self.old_tm.tm_hour))
            .replace("H", &self.old_tm.tm_hour.to_string())
            // .replace("hh", &format!("{:02}", if self.tm_hour > 11 {
            //                                      self.tm_hour - 12
            //                                  } else {
            //                                      self.tm_hour
            //                                  } + 1))
            // .replace("h", &format!("{}", if self.tm_hour > 11 {
            //                                  self.tm_hour - 12
            //                              } else {
            //                                  self.tm_hour
            //                              } + 1))
            .replace("mm", &format!("{:02}", self.old_tm.tm_min))
            .replace("m", &self.old_tm.tm_min.to_string())
            .replace("ns", &self.old_tm.tm_nsec.to_string())
            .replace("ss", &format!("{:02}", self.old_tm.tm_sec))
            .replace("s", &self.old_tm.tm_sec.to_string())
    }

    pub fn timestamp(&self) -> time::Timespec {
        self.old_tm.to_timespec()
    }
}   

    // /// Returns the number of seconds since January 1, 1970 UTC
    

    // /// Returns true if the year is a leap year
    // pub fn is_leap(&self) -> bool {
    //     is_persian_leap(self.tm_year)
    // }

    // /// Convert time to the local timezone
    // pub fn to_local(&self) -> Time {
    //     match self.tm_utcoff {
    //         0 => at(self.to_timestamp()),
    //         _ => *self
    //     }
    // }

    // /// Convert time to the UTC
    // pub fn to_utc(&self) -> Time {
    //     match self.tm_utcoff {
    //         0 => *self,
    //         _ => at_utc(self.to_timestamp())
    //     }
    // }

    /// Returns the formatted representation of time
    ///     YYYY     year (e.g. 1394)
    ///     YY               2-digits representation of year (e.g. 94)
    ///     MMM              the Persian name of month (e.g. JAN)
    ///     MM               2-digits representation of month (e.g. 01)
    ///     M                month (e.g. 1)
    ///     DD               day of year (starting from 1)
    ///     D                day of year (starting from 0)
    ///     dd               2-digits representation of day (e.g. 01)
    ///     d                day (e.g. 1)
    ///     E                the Persian name of weekday (e.g. شنبه)
    ///     e                the Persian short name of weekday (e.g. ش)
    ///     A                the Persian name of 12-Hour marker (e.g. قبل از ظهر)
    ///     a                the Persian short name of 12-Hour marker (e.g. ق.ظ)
    ///     HH               2-digits representation of hour [00-23]
    ///     H                hour [0-23]
    ///     kk               2-digits representation of hour [01-24]
    ///     k                hour [1-24]
    ///     hh               2-digits representation of hour [01-12]
    ///     h                hour [1-12]
    ///     KK               2-digits representation of hour [00-11]
    ///     K                hour [0-11]
    ///     mm               2-digits representation of minute [00-59]
    ///     m                minute [0-59]
    ///     ss               2-digits representation of seconds [00-59]
    ///     s                seconds [0-59]
    ///     ns               nanoseconds
    


/// Creates an empty `ptime::Tm`
// pub fn empty_tm() -> Time {
//     Time {
//         old_tm: None,
//         tm_sec: 0,
//         tm_min: 0,
//         tm_hour: 0,
//         tm_mday: 0,
//         tm_mon: 0,
//         tm_year: 0,
//         tm_wday: 0,
//         tm_yday: 0,
//         tm_isdst: 0,
//         tm_utcoff: 0,
//         tm_nsec: 0,
//     }
// }

// /// Converts Gregorian calendar to Persian calendar
// pub fn from_gregorian(gregorian_tm:time::Tm) -> Time {
//     let mut year: i32;
//     let gy = gregorian_tm.tm_year + 1900;
//     let gm = gregorian_tm.tm_mon + 1;
//     let gd = gregorian_tm.tm_mday;

//     let jdn: i32 = if gy > 1582 || (gy == 1582 && gm > 10) || (gy == 1582 && gm == 10 && gd > 14) {
//         ((1461 * (gy + 4800 + ((gm - 14) / 12))) / 4) + ((367 * (gm - 2 - 12*((gm-14)/12))) / 12) - ((3 * ((gy + 4900 + ((gm - 14) / 12)) / 100)) / 4) + gd - 32075
//     } else {
//         367 * gy - ((7 * (gy + 5001 + ((gm - 9) / 7))) / 4) + ((275 * gm) / 9) + gd + 1729777
//     };

//     let dep = jdn - get_jdn(475, 1, 1);
//     let cyc = dep / 1029983;
//     let rem = dep % 1029983;
//     let ycyc = if rem == 1029982 {
//         2820
//     } else {
//         let a = rem / 366;
//         (2134 * a + 2816 * (rem % 366) + 2815) / 1028522 + a + 1
//     };

//     year = ycyc + 2820 * cyc + 474;
//     if year <= 0 {
//         year -= 1;
//     }

//     let dy: f64 = (jdn - get_jdn(year, 1, 1) + 1) as f64;
//     let month: i32 = if dy <= 186f64 {
//         let mod_dy: f64 = dy / 31f64;
//         mod_dy.ceil() as i32
//     } else {
//         let mod_dy: f64 = (dy - 6f64) / 30f64;
//         mod_dy.ceil() as i32
//     } - 1;
//     let day = jdn - get_jdn(year, month + 1, 1) + 1;

//     Time {
//         old_tm: Some(gregorian_tm),
//         tm_sec: gregorian_tm.tm_sec,
//         tm_min: gregorian_tm.tm_min,
//         tm_hour: gregorian_tm.tm_hour,
//         tm_mday: day,
//         tm_mon: month,
//         tm_year: year,
//         tm_wday: get_persian_weekday(gregorian_tm.tm_wday),
//         tm_yday: get_persian_yday(month, day),
//         tm_isdst: gregorian_tm.tm_isdst,
//         tm_utcoff: gregorian_tm.tm_utcoff,
//         tm_nsec: gregorian_tm.tm_nsec,
//     }
// }

// /// Creates a new instance of Persian time from Gregorian date
// pub fn from_gregorian_date(g_year: i32, g_month: i32, g_day: i32) -> Option<Time> {
//     from_gregorian_components(g_year, g_month, g_day, 0, 0, 0, 0)
// }

// /// Creates a new instance of Persian time from Persian date
// pub fn from_persian_date(p_year: i32, p_month: i32, p_day: i32) -> Option<Time> {
//     from_persian_components(p_year, p_month, p_day, 0, 0, 0, 0)
// }

// /// Creates a new instance of Persian time from Gregorian date components
// pub fn from_gregorian_components(g_year: i32, g_month: i32, g_day: i32, hour: i32, minute: i32, second: i32, nanosecond: i32) -> Option<Time> {
//     if is_time_valid(hour, minute, second, nanosecond) && is_gregorian_date_valid(g_year, g_month, g_day) {
//         let tm = time::Tm{
//             tm_sec: second,
//             tm_min: minute,
//             tm_hour: hour,
//             tm_mday: g_day,
//             tm_mon: g_month,
//             tm_year: g_year - 1900,
//             tm_wday: 0,
//             tm_yday: 0,
//             tm_isdst: 0,
//             tm_utcoff: 0,
//             tm_nsec: nanosecond,
//         };
//         return Some(at_utc(tm.to_timespec()))
//     }
//     None
// }

// /// Creates a new instance of Persian time from Persian date components
// // FIXME: Calculate the weekday without converting to Gregorian calendar
// pub fn from_persian_components(p_year: i32, p_month: i32, p_day: i32, hour: i32, minute: i32, second: i32, nanosecond: i32) -> Option<Time> {
//     if is_time_valid(hour, minute, second, nanosecond) && is_persian_date_valid(p_year, p_month, p_day) {
//         let mut tm = Time{
//             tm_sec: second,
//             tm_min: minute,
//             tm_hour: hour,
//             tm_mday: p_day,
//             tm_mon: p_month,
//             tm_year: p_year,
//             tm_wday: 0,
//             tm_yday: get_persian_yday(p_month, p_day),
//             tm_isdst: 0,
//             tm_utcoff: 0,
//             tm_nsec: nanosecond,
//             old_tm: None
//         };
//         let old_tm = time::at_utc(tm.to_timestamp());
//         tm.tm_wday = get_persian_weekday(old_tm.tm_wday);
//         tm.old_tm = Some(old_tm);
//         return Some(tm)
//     }
//     None
// }

// /// Creates a new instance of Persian time from the number of seconds since January 1, 1970 in UTC
// pub fn at_utc(clock: time::Timespec) -> Time {
//     from_gregorian(time::at_utc(clock))
// }

// /// Creates a new instance of Persian time from the number of seconds since January 1, 1970 in the local timezone
// pub fn at(clock: time::Timespec) -> Time {
//     from_gregorian(time::at(clock))
// }

// /// Creates a new instance of Persian time corresponding to the current time in UTC
// pub fn now_utc() -> Time {
//     from_gregorian(time::now_utc())
// }

/// 
pub fn now() -> Time {
    Time::new(time::now())
}


// fn get_persian_weekday(wd: i32) -> i32 {
//     match wd {
//         0 => 1,
//         1 => 2,
//         2 => 3,
//         3 => 4,
//         4 => 5,
//         5 => 6,
//         6 => 0,
//         _ => panic!("invalid weekday value of {}", wd),
//     }
// }

// fn get_gregorian_weekday(wd: i32) -> i32 {
//     match wd {
//         0 => 6,
//         1 => 0,
//         2 => 1,
//         3 => 2,
//         4 => 3,
//         5 => 4,
//         6 => 5,
//         _ => panic!("invalid weekday value of {}", wd),
//     }
// }



fn is_gregorian_leap(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}


fn is_date_valid(year: i32, month: i32, day: i32) -> bool {
    if month < 0 || month > 11 {
        return false
    }

    [
        [31, 31],
        [28, 29],
        [31, 31],
        [30, 30],
        [31, 31],
        [30, 30],
        [31, 31],
        [31, 31],
        [30, 30],
        [31, 31],
        [30, 30],
        [31, 31],
    ][month as usize][is_gregorian_leap(year) as usize] >= day
}

fn is_time_valid(hour: i32, minute: i32, second: i32, nanosecond: i32) -> bool {
    !(hour < 0 || hour > 23 || minute < 0 || minute > 59 || second < 0 || second > 59 || nanosecond < 0 || nanosecond > 999999999)
}