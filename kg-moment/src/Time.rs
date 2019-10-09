extern crate time;

use time::*;

pub struct Time {
    tm: Tm,
    pub year: i32,
}

impl Time {
    fn init(&mut self){
        self.year = self.tm.tm_year+1990;
    }
}

fn parse_by_tm(tm: Tm) -> Time {
    let mut t = Time { tm: tm, year: 0 };
    t.init();

    return t;
}

pub fn now() -> Time {
    let t = parse_by_tm(time::now());

    return t;
}


