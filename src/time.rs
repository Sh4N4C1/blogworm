use chrono::{NaiveDate, TimeZone, Utc, NaiveDateTime, DateTime};

#[allow(warnings)]
pub fn parse_time(postid: u32, html_time: String) -> u64 {
    if postid == 1 || postid == 9{
        let format_str = "%B %e, %Y";
        let date = NaiveDate::parse_from_str(html_time.as_str(), format_str).unwrap();
        let datetime = Utc.from_local_date(&date).unwrap().and_hms_opt(0, 0, 0).unwrap();
        let timestamp = datetime.timestamp();
        timestamp.try_into().unwrap()
    }else if postid == 2{
       let date_str = html_time.split("- ").nth(1).map(str::trim); 
       if let Some(date_str) = date_str {
           let date = NaiveDate::parse_from_str(date_str, "%d/%m/%Y").unwrap();
           let datetime = Utc.from_local_date(&date).unwrap().and_hms_opt(0, 0, 0).unwrap();
           let timestamp = datetime.timestamp();
           timestamp.try_into().unwrap()
       }else{0}
       
    }else if postid == 3{
        let datetime = NaiveDateTime::parse_from_str(&html_time,"%Y-%m-%dT%H:%M:%S%.3fZ").unwrap();
        datetime.timestamp().try_into().unwrap()

    }else if postid == 4{
        let datetime = NaiveDateTime::parse_from_str(&html_time, "%Y-%m-%dT%H:%M:%S").unwrap();
        datetime.timestamp().try_into().unwrap()
    }else if postid == 5{
//        let datetime = NaiveDateTime::parse_from_str(&html_time, "%Y-%m-%dT%H:%M:%S").unwrap();
        let datetime: DateTime<Utc> = html_time.parse().unwrap();
        datetime.timestamp().try_into().unwrap()
    }else if postid == 6{
        //println!("!!!!!!!!!!!{}",&html_time.replace("th", "").replace("st", "").replace("nd", "").replace("rd", ""));
        let date = NaiveDate::parse_from_str(&html_time.replace("th,", "").replace("st,", "").replace("nd,", "").replace("rd,", ""), "%B %e %Y").unwrap();
        let datetime: DateTime<Utc> = DateTime::from_utc(date.and_hms(0, 0, 0), Utc);
        datetime.timestamp().try_into().unwrap()

    }else if postid == 10 {
        let date = NaiveDate::parse_from_str(&html_time, "%d %b %Y").unwrap();
        let datetime = Utc.from_local_date(&date).unwrap().and_hms_opt(0, 0, 0).unwrap();
        datetime.timestamp().try_into().unwrap()
    }else{0}
}
