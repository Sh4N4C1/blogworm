use reqwest;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use tokio::task;
use std::error::Error;
use indicatif::ProgressBar;
use scraper::{Html, Selector};
use chrono::{DateTime, Local, TimeZone};
use std::io::{self, Read, Write};
use std::fs::{self, File};
use blogworm::Postsrc;
use blogworm::{Post, Show};
use super::time::parse_time;
use std::collections::HashMap;
use serde_json;


pub async fn send_request(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let reponse = reqwest::get(url.to_string()).await?;
    let body = reponse.text().await?;
    Ok(body)
}

pub fn parse_postsrc(document_body: &str, link_class: &str, post_id: u32) -> Result<Vec<String>, Box<dyn std::error::Error>>{
    if post_id == 2 {
        let document = Html::parse_document(&document_body);
        let div_selector = Selector::parse("div.link").unwrap();
        let a_selector = Selector::parse("a").unwrap();
        let a_values: Vec<String> = document
            .select(&div_selector)
            .flat_map(|div| div.select(&a_selector))
            .filter_map(|a| a.value().attr("href").map(String::from))
            .collect();

		Ok(a_values)
    }else{
        let document = Html::parse_document(&document_body);
        let a_selector = Selector::parse(&format!("a.{}", link_class)).unwrap();
        let a_values: Vec<String> = document 
            .select(&a_selector)
            .filter_map(|a| a.value().attr("href").map(String::from))
            .collect();
		Ok(a_values)
  
    }
}

pub fn parse_post(document_body: &str, time_class: &str, title_class: &str, author_class: &str, content_class: &str, post_id: u32, post_url: String) -> Result<Post, Box<dyn std::error::Error>>{
    let document = Html::parse_document(&document_body);
    let time_values: Vec<String> = handle_parse_post(time_class, &document);
    let title_values: Vec<String> = handle_parse_post(title_class, &document);
    let author_values: Vec<String> = handle_parse_post(author_class, &document);
    let content_values: Vec<String> = handle_parse_post(content_class, &document);
    //if post_id == 2 {
     //   println!("{}", time_values[0]);
      //  println!("{}", title_values[0]);
        //println!("{}", author_values[0]);
        //println!("{}", content_values[0]);
//        println!("[CHECK] time: {}\ntitle: {}\nauthor: {}\ncontent:{}\n", time_values[0],title_values[0],author_values[0],content_values[0]);
    //}
    let parsed_time = parse_time(post_id, time_values[0].clone());
    Ok(Post {content: content_values[0].clone(), author: author_values[0].clone(), title: title_values[0].clone(), create_timestamp: parsed_time, url: post_url})
}
pub fn handle_parse_post(class_name: &str, document: &Html) -> Vec<String>{
    let abc_selector = Selector::parse(class_name).unwrap();
    document.select(&abc_selector).map(|a| a.inner_html()).collect()
    
}
pub async fn get_blog_link_from_postsrc(postsrc: &Postsrc) -> Result<(String, Vec<String>), Box<dyn std::error::Error>>{
    let website = &postsrc.website;
    let link_class = &postsrc.link_class;
    let body = send_request(website.to_string()).await;

    match body {
          Ok(document_body) => {
            //println!("[*] Success Get {}", website);
            let a_link_list  = parse_postsrc(document_body.as_str(), link_class.as_str(), postsrc.postsrc_id);
            match a_link_list {
                Ok(a_values) => {
                    //for iteam in &a_values{
                     //   println!("[*] {} POST BLOG: {}", website, iteam);
                    //}
                    Ok((website.clone(), a_values))
                }
                Err(error) => {
                    println!("[!] Fail to parse HTML.\n Error: {}", error);
                    return Err(error);
                }
            }
          }
          Err(error) => {
                eprintln!("Error: {}",error);
                return Err(error);
          }

    }

}

pub async fn get_post_from_link(post_url: String, postsrc: &Postsrc) -> Result<Post, Box<dyn std::error::Error>>{
    let body = send_request(post_url.to_string()).await;
    match body {
        Ok(document_body) => {
            //println!("[*] Success Get {}", post_url);
            let post = parse_post(document_body.as_str(), postsrc.time_class.as_str(), postsrc.title_class.as_str(), postsrc.author_class.as_str(), postsrc.content_class.as_str(), postsrc.postsrc_id, post_url).unwrap();
            //post.show_post();
            Ok(post)

        }
        Err(error) => {
            println!("[!] Failt to parse POST HTML.\n Error: {}",error);
            return Err(error);
        }

    }
}

#[warn(deprecated)]
pub fn timestamp_to_readable(timestamp: u64) -> DateTime<Local>{
    let timestamp = chrono::NaiveDateTime::from_timestamp(timestamp as i64, 0);
    Local.from_utc_datetime(&timestamp)
}


pub fn save_new_post_to_file(new_post_list: Vec<Post>, save_path: &str) -> Result<(), Box<dyn std::error::Error>>{
    let json = serde_json::to_string(&new_post_list).unwrap();
    let mut file = File::create(save_path).expect("Failed to create file.");
    file.write_all(json.as_bytes()).expect("Failed to write to file.");
    //let mut homedir = dirs::home_dir().expect("Fail to get home dir");
    //homedir.push(".blogworm");
    //std::fs::create_dir_all(&homedir).expect("Fail to create dir");
    //homedir.push("new_post.json");
    Ok(())

}








