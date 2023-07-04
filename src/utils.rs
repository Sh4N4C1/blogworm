use reqwest;
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashSet;
use chrono::{DateTime, Local, TimeZone};
use std::io::{Write};
use std::fs::{File};
use blogworm::Postsrc;
use blogworm::{Post};
use super::time::parse_time;
use serde_json;

pub async fn send_request(url: String) -> Result<String, Box<dyn std::error::Error>> {
    //println!("[DEBUG] {}",url);
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
    }else if post_id == 3{
        let document = Html::parse_document(&document_body);
        let div_selector = Selector::parse("div.l:not([class*= ' '])").unwrap();
        let a_selector = Selector::parse(&format!("a.{}:not([href*='/tag/pentest*'])", link_class)).unwrap();

        let a_values: Vec<String> = document
            .select(&div_selector)
            .filter(|div| {
                let classes = div.value().attr("class").unwrap_or("");
                classes.split_whitespace().all(|class| class == "l")
            })
            .flat_map(|div| div.select(&a_selector))
            .filter_map(|a| {

                let href = a.value().attr("href").unwrap_or("");
                if href.starts_with("/@") && href.matches('/').count() == 2{
                    Some(extract_path(href).unwrap().to_owned())
                }else{
                    None
                }
            }) 
            .collect();
       // println!("[!]DEBUG: {:?}",a_values);
        Ok(a_values.into_iter().collect::<HashSet<String>>().into_iter().collect())

    }else if post_id == 4{
        let re = Regex::new(r#"href="([^"]+)""#).unwrap();
        let document = Html::parse_document(&document_body);
        let a_selector = Selector::parse("noscript").unwrap();
        let a_values: Vec<String> = document
            .select(&a_selector)
            .map(|a| a.inner_html())
            .collect();
        //println!("{:?}",a_values);
       // println!("{:?}",a_values[0].as_str());
        let mut paths = Vec::new();
        for capture in re.captures_iter(a_values[0].as_str()){
           if let Some(href) = capture.get(1) {
               let value = href.as_str();
               paths.push(value.to_string());
           } 
        }
//        println!("{:?}", paths);

		Ok(paths)

    }else if post_id == 5 || post_id == 8{
        if post_id == 8 {
            let document = Html::parse_document(&document_body);
            let a_selector = Selector::parse(link_class).unwrap();
            let a_values: Vec<String> = document
                .select(&a_selector)
                .filter_map(|a|{
            let a_result = a.value().attr("href").unwrap();
            let path = a_result.split('/').filter(|&s| !s.is_empty()).last().unwrap().to_string() + "/";
            Some(path)
            }).collect();
            return Ok(a_values);

        }
        let document = Html::parse_document(&document_body);
        let a_selector = Selector::parse(link_class).unwrap();
        let a_values: Vec<String> = document
            .select(&a_selector)
            .filter_map(|a|{
                let a_result = a.value().attr("href").unwrap();
                let path = a_result.split('/').filter(|&s| !s.is_empty()).last().unwrap().to_string();
                Some(path)
            }).collect();
        Ok(a_values)
    }else if post_id ==10 {
        let document = Html::parse_document(&document_body);
        let a_selector = Selector::parse(link_class).unwrap();
        let a_values: Vec<String> = document 
            .select(&a_selector)
            .filter_map(|a| {
                let data_url = a.value().attr("data-url").map(String::from);
                if let Some(url) = &data_url {
                    if url.contains("2023"){
                        data_url
                    }else {
                        None
                    }
                }else {
                    None
                }
            })
            .collect();
  //      println!("{:?}",a_values);
		return Ok(a_values);

    }else{
 //       println!("{}",&document_body);
        let document = Html::parse_document(&document_body);
        let a_selector = Selector::parse(link_class).unwrap();
        let a_values: Vec<String> = document 
            .select(&a_selector)
            .filter_map(|a| a.value().attr("href").map(String::from))
            .collect();
  //      println!("{:?}",a_values);
		Ok(a_values)
  
    }
}

pub fn parse_post(document_body: &str, time_class: &str, title_class: &str, author_class: &str, content_class: &str, post_id: u32, post_url: String) -> Result<Post, Box<dyn std::error::Error>>{
    let document = Html::parse_document(&document_body);
    let time_values: Vec<String> = handle_parse_post(time_class, &document, post_id);
    let title_values: Vec<String> = handle_parse_post(title_class, &document, post_id);
    let author_values: Vec<String> = handle_parse_post(author_class, &document, post_id);
    let content_values: Vec<String> = handle_parse_post(content_class, &document, post_id);
 //println!("[DEBUG] {:?}",content_values);
  //println!("[DEBUG] {:?}",time_values[0]);
  //println!("[DEBUG] {:?}",author_values);
//println!("[DEBUG] {:?}",title_values);
    if post_id == 6 {
 //       println!("!!!{}!!!", time_values[time_values.len()-1]);
        let parsed_time = parse_time(post_id, time_values[time_values.len()-1].clone());
        return Ok(Post {content: content_values[0].clone(), author: author_values[0].clone(), title: title_values[0].clone(), create_timestamp: parsed_time, url: post_url});
    }else if post_id == 10{
        let parsed_time = parse_time(post_id, time_values[0].clone());
        return Ok(Post {content: content_values[0].clone(), author: author_values[0].clone(), title: title_values[0].clone(), create_timestamp: parsed_time, url: post_url});

    }

    let parsed_time = parse_time(post_id, time_values[0].clone());
    Ok(Post {content: content_values[0].clone(), author: author_values[0].clone(), title: title_values[0].clone(), create_timestamp: parsed_time, url: post_url})
}
pub fn handle_parse_post(class_name: &str, document: &Html, post_id: u32) -> Vec<String>{
    if post_id == 3 || post_id == 4 || post_id == 8 || post_id == 9{
        if class_name == "p.lead.my-3"{
            let abc_selector = Selector::parse(class_name).unwrap();
            let a_values :Vec<String>= document.select(&abc_selector).map(|a| a.inner_html()).collect();
            return a_values;
            
        }
        let abc_selector = Selector::parse(class_name).unwrap();
        document.select(&abc_selector).filter_map(|a| a.value().attr("content").map(String::from)).collect()
    }else if post_id == 5 || post_id == 7{
        if  class_name == "a.author.url.fn" {
            let abc_selector = Selector::parse(class_name).unwrap();
            document.select(&abc_selector).map(|a| a.inner_html()).collect()
        }else if class_name == "time.entry-date.published" || class_name == "time"{
            let abc_selector = Selector::parse(class_name).unwrap();
            document.select(&abc_selector).filter_map(|a| a.value().attr("datetime").map(String::from)).collect()
        }else {
            let abc_selector = Selector::parse(class_name).unwrap();
            document.select(&abc_selector).filter_map(|a| a.value().attr("content").map(String::from)).collect()
        }
    }else if post_id == 6 {
        if class_name == "div.flex.flex-wrap.items-center.gap-x-2.font-semibold"
        {
            let abc_selector = Selector::parse(class_name).unwrap();
            let div_selector = Selector::parse("span:not([class*= 'text-subtle.mx-2'])").unwrap();
            let a_values: Vec<String> = document
              .select(&abc_selector)
              .flat_map(|div| div.select(&div_selector))
              .map(|a| a.inner_html())
              .collect();
            a_values

        }else if class_name == "div.rich-text" {
            let abc_selector = Selector::parse(class_name).unwrap();                                                                                                
            let h2_selector = Selector::parse("h2").unwrap();
            let content_values: Vec<String>  = document
                .select(&abc_selector).flat_map(|div| div.select(&h2_selector))
                .map(|a| a.inner_html())
                .collect();
            content_values


        }else {
            let abc_selector = Selector::parse(class_name).unwrap();
            document.select(&abc_selector).filter_map(|a| a.value().attr("content").map(String::from)).collect()

        }
    }else if post_id == 10{
        if class_name == "div.post > p"{
            let abc_selector = Selector::parse(class_name).unwrap();
            let first_p_inner_html: Option<String> = document
                .select(&abc_selector)
                .next()
                .map(|p| p.inner_html());
            return first_p_inner_html.into_iter().collect::<Vec<String>>();
        }else if class_name == "h1.post-title"{
            let abc_selector = Selector::parse(class_name).unwrap();
            let a_values :Vec<String>= document.select(&abc_selector).map(|a| a.inner_html()).collect();
            return a_values;

        }else if class_name == "span.post-date"{

              let a_selector = Selector::parse(class_name).unwrap();
              let a_values: Vec<String> = document
                  .select(&a_selector)
                  .filter_map(|a|{
              let a_result = a.inner_html();
              let path = a_result.split("Posted by ").filter(|&s| !s.is_empty()).last().unwrap().to_string() ;
              Some(path)
              }).collect();
              return a_values;
        }else {
            let a_selector = Selector::parse("span.post-date").unwrap();
            let a_values: Vec<String> = document
                    .select(&a_selector)
                    .filter_map(|a|{
            let a_result = a.inner_html();
            let path = a_result.split(" - Posted by ").filter(|&s| !s.is_empty()).next().unwrap().to_string();
            Some(path)
            }).collect();
            return a_values;
        }
    }else {
        let abc_selector = Selector::parse(class_name).unwrap();
        document.select(&abc_selector).map(|a| a.inner_html()).collect()
    }
    
}
pub async fn get_blog_link_from_postsrc(postsrc: &Postsrc) -> Result<(String, Vec<String>), Box<dyn std::error::Error>>{
    let website = &postsrc.website;
    let link_class = &postsrc.link_class;
    let body = send_request(website.to_string()).await;

    match body {
          Ok(document_body) => {
            let a_link_list  = parse_postsrc(document_body.as_str(), link_class.as_str(), postsrc.postsrc_id);
            match a_link_list {
                Ok(a_values) => {
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
            let post = parse_post(document_body.as_str(), postsrc.time_class.as_str(), postsrc.title_class.as_str(), postsrc.author_class.as_str(), postsrc.content_class.as_str(), postsrc.postsrc_id, post_url).unwrap();
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
    let timestamp = chrono::NaiveDateTime::from_timestamp_opt(timestamp as i64, 0).unwrap();
    Local.from_utc_datetime(&timestamp)
}


pub fn save_new_post_to_file(new_post_list: Vec<Post>, save_path: &str) -> Result<(), Box<dyn std::error::Error>>{
    let json = serde_json::to_string(&new_post_list).unwrap();
    let mut file = File::create(save_path).expect("Failed to create file.");
    file.write_all(json.as_bytes()).expect("Failed to write to file.");
    Ok(())

}

pub async fn get_single_post_handle(postsrc: &Postsrc) -> Result<Vec<String>, Box<dyn std::error::Error>>{

    match get_blog_link_from_postsrc(&postsrc).await {
        Ok(result) => {
            let (website, mut post_list) = result;
            for post in post_list.iter_mut(){
                let temp_url = website.split('/').take(3).collect::<Vec<&str>>().join("/");
                if post.starts_with('/'){
                     *post = crate::replace_second_slash(&(temp_url +"/" + post)); 
                }
                else{
                     *post = crate::replace_second_slash(&(temp_url +"//" + post));
                }

            };
            Ok(post_list)
        }
        Err(error) => {
            eprintln!("Error: {}",error);
            return Err(error)
        }
    }
    
}
pub fn check_name(postname: String) -> Option<Postsrc>{
    let _flag =false;
    for postsrc in crate::POSTSRC_LIST.iter() {
        if postname == postsrc.name {
            let name = &postsrc.name;
            let _postsrc_id = &postsrc.postsrc_id;
            let link_class = &postsrc.link_class;
            let website = &postsrc.website;
            let author_class = &postsrc.author_class;
            let content_class = &postsrc.content_class;
            let time_class = &postsrc.time_class;
            let title_class = &postsrc.title_class;
            let result :Postsrc = Postsrc{name: name.to_string(), postsrc_id: postsrc.postsrc_id, website: website.to_string(), link_class: link_class.to_string(), author_class: author_class.to_string(), content_class: content_class.to_string(), title_class: title_class.to_string(), time_class: time_class.to_string()};
            return Some(result);
        } 
    }
    None
}

fn extract_path(url: &str) -> Option<&str> {
    if let Some(query_start) = url.find('?') {
        Some(&url[..query_start])
    } else {
        None
    }
}
