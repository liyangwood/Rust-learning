use std::cmp::Ordering;
use std::ops::{Add, Sub};
use std::fmt;

use ::time;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Moment {
    pub old_tm: time::Tm,

    pub year: i32
}

impl fmt::Display for Moment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string("YYYY-MM-DD HH:mm:ss"))
    }
}

impl Moment {
    pub fn new(tm:time::Tm) -> Moment {
        let mut rs = Moment {
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

    pub fn get_timestamp(&self) -> i64 {
        let rs:time::Timespec = self.old_tm.to_timespec();
        
    }
} 

pub fn now() -> Moment {
    Moment::new(time::now())
}