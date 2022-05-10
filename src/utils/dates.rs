use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

pub fn now_ndt()-> NaiveDateTime{
  let d = NaiveDate::from_ymd(2015, 6, 3);
  let t = NaiveTime::from_hms_milli(12, 34, 56, 789);
  NaiveDateTime::new(d,t)
}