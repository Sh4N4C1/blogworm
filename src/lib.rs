use lazy_static::lazy_static;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local, TimeZone};
use colored::Colorize;

lazy_static! {
    static ref BLOG_LIST: Vec<&'static str> = vec![
        "https://www.sonarsource.com/blog/",
        "https://www.synacktiv.com/publications",
    ];

    pub static ref POSTSRC_LIST: Vec<Postsrc> = vec![
        Postsrc {
            postsrc_id: 1,
            name: "sonarsource".to_string(),
            website: "https://www.sonarsource.com/blog/".to_string(),
            link_class: "css-1ovxvot.e1yv2nyc5".to_string(),
            author_class: "p.css-2onphd.efy4oos3".to_string(),
            title_class: "h1.css-g2zbjf.e1ydtod09".to_string(),
            content_class: "div[class*=\"css-c07j1o\"][class*=\"egkxqwt0\"] > p".to_string(),
            time_class: "time.css-2onphd.e1ydtod05".to_string(),
        },
        Postsrc {
            postsrc_id: 2,
            name: "synacktiv".to_string(),
            website: "https://www.synacktiv.com/publications".to_string(),
            link_class: "link".to_string(),
            author_class: "span.author-name".to_string(),
            time_class: "div.short-info".to_string(),
            title_class: "span[property='schema:name']".to_string(),
            content_class: "div.introduction".to_string(),

        }
    ];
}
pub trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Serialize, Deserialize)]
pub struct Postsrc {
    pub name: String,
    pub postsrc_id: u32,
    pub website: String,
    pub link_class: String,
    pub time_class: String,
    pub title_class: String,
    pub author_class: String,
    pub content_class: String,

}

impl Summary for Postsrc {
    fn summarize(&self) -> String {
        format!("[{}] {}", self.name,self.website)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub create_timestamp: u64,
    pub title: String,
    pub author: String,
    pub content: String,
    pub url: String,
}

pub trait Show {
    fn show_post(&self) ;
}

#[warn(deprecated)]
pub fn timestamp_to_readable(timestamp: u64) -> DateTime<Local>{
    let timestamp = chrono::NaiveDateTime::from_timestamp(timestamp as i64, 0);
    Local.from_utc_datetime(&timestamp)
}

impl Show for Post {
    fn show_post(&self){
        print!("\r[*] Title: {}\n", self.title.blue());
        print!("[*] Author: {}\n",self.author.red());
        print!("[*] Time: {}\n",self.create_timestamp.to_string().red());
        print!("[*] Contents: {}\n",self.content.blue());

//        println!("[+] Post Title: {}\n[+] Post author: {}\n[+] Post create time: {}\n[+] Post content: {}\n", self.title.red().bold().normal().clear(), self.author.red().bold(), self.create_timestamp.to_string().normal().clear(), self.content.normal().clear());

    }
}
