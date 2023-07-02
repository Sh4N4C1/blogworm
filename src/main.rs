mod config;
mod time;
use std::sync::{Arc, Mutex};
use console::{style, Emoji};
use colored::*;
mod utils;
use blogworm::Summary;
use blogworm::{Postsrc, Post};
use indicatif::{ProgressBar,ProgressStyle};
use blogworm::POSTSRC_LIST;
use tokio::task;
use utils::timestamp_to_readable;
use std::fs::{self, File};
use std::collections::HashMap;
use std::io::{self, Read, Write};
//TODO: rewrite html class [*]
//TODO: update process bar
//TODO: store post time and compare ~/.blogworm/timestamp
//
//
static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: config::args::Config =  config::args::get_configs().unwrap();

    let (last_timestamp, current_timestamp) = checktimestamp().unwrap();
    let hashmap: HashMap<Vec<String>, &Postsrc> = HashMap::new();
    let hashmap = Arc::new(Mutex::new(hashmap));
    
    
    
    let mut tasks = vec![];
    println!("[+] {} Starting update post link...",TRUCK);
    for postsrc in POSTSRC_LIST.iter() {
        let hashmap_clone = Arc::clone(&hashmap);
        let postsrc_clone = Arc::new(postsrc.clone());
        println!("[*]{}PostSrc: {}", CLIP, postsrc.summarize());

        let task = task::spawn(async move {
            match utils::get_blog_link_from_postsrc(postsrc).await {
                Ok(result) => {
                    let (website, mut post_list) = result;
                    let mut result_list = 
                    for post in post_list.iter_mut(){
                        let temp_url = website.split('/').take(3).collect::<Vec<&str>>().join("/");
                        if !temp_url.ends_with('/'){
                            *post = temp_url + "/" + post;
                        }else{
                            *post = temp_url + post;
                        }
                    };
                    let mut hashmap = hashmap_clone.lock().unwrap();
                    hashmap.insert(post_list, &postsrc_clone);
                    
                }
                Err(error) => {
                    eprintln!("Error: {}", error);
                }
            }
        });
        tasks.push(task);

    }
    let bar = ProgressBar::new(tasks.len().try_into().unwrap());
    for task in tasks {
        bar.inc(1);
        task.await.expect("Failed to join task") 
    }
    println!("[*] Success update All PosrtSrc! {}", SPARKLE);
    bar.finish();

    let  hashmap  = hashmap.lock().unwrap();
    let mut blog_tasks = vec![];
    let total_vec_count: usize = hashmap.keys().map(|vec| vec.len()).sum();

    let bar = ProgressBar::new(total_vec_count.try_into().unwrap());
    let bar = Arc::new(Mutex::new(ProgressBar::new(total_vec_count.try_into().unwrap())));
    bar.lock().unwrap().set_style(ProgressStyle::with_template("[{pos}/{len}] {spinner} {msg}").unwrap().tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ "));
    let new_post_list: Arc<Mutex<Vec<Post>>> = Arc::new(Mutex::new(vec![]));


    for (post_list, postsrc) in hashmap.clone(){

        for post in post_list {
            let progress_clone = Arc::clone(&bar);
            let new_post_list_clone = Arc::clone(&new_post_list);
            let task = task::spawn(async move { match utils::get_post_from_link(post, postsrc).await {
                Ok(post) => {
                    progress_clone.lock().unwrap().set_message(format!("\n[+] {} Post Title: {}\n[+] {} Post author: {}",PAPER, post.title.red().bold(), LOOKING_GLASS, post.author.blue().bold()));
                    progress_clone.lock().unwrap().inc(1);
                    if post.create_timestamp > last_timestamp {
                        new_post_list_clone.lock().unwrap().push(post);

                    }
                    
                }
                Err(error) => {
                    eprintln!("Error: {}",error);
                }
            }});    
            blog_tasks.push(task);

        }
    }
    for task in blog_tasks{
        task.await.expect("Failed to join task");
    }
    bar.lock().unwrap().finish();
    for new_post in new_post_list.lock().unwrap().iter(){
        println!("[New] {}\n[Time] {}\n",new_post.url, new_post.create_timestamp.to_string());
    }
    
    if new_post_list.lock().unwrap().len() == 0 {
        println!("[*] Not found new Post! :)")

    }else {
        

        let save_path = config.get_save_path();
        let clonelist = convert_arc_mutex_to_vec(new_post_list);
        utils::save_new_post_to_file(clonelist,save_path);
            
    }
    Ok(())
}

fn checktimestamp() -> io::Result<(u64, u64)>{

    let home_dir = dirs::home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Can't get user hoem dir"))?;
    let mut app_dir = home_dir.clone();
    app_dir.push(".blogworm");
    fs::create_dir_all(&app_dir)?;

    let mut timestamp_file = app_dir.clone();
    timestamp_file.push("timestamp");

    let current_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to get system time")
        .as_secs();

    if !timestamp_file.exists() {
        let mut file = File::create(&timestamp_file)?;
        file.write_all(current_timestamp.to_string().as_bytes())?;
        println!("[*] It seems you frist times to run blogworm or timestamp file deleted");
        let datetime = timestamp_to_readable(current_timestamp);
        println!("[*] Current run blogworm AT: {}", datetime.format("%Y-%m-%d %H:%M:%S"));
        Ok((0, current_timestamp))

    } else {
        let mut file = File::open(&timestamp_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let mut file = File::create(&timestamp_file)?;
        file.write_all(current_timestamp.to_string().as_bytes())?;
        let last_timestamp = contents.trim().parse::<u64>().map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

        let mut datetime = timestamp_to_readable(last_timestamp);
        println!("[*] Last run blogworm AT: {}", datetime.format("%Y-%m-%d %H:%M:%S"));
        datetime = timestamp_to_readable(current_timestamp);
        println!("[*] Current run blogworm AT: {}", datetime.format("%Y-%m-%d %H:%M:%S"));
        Ok((last_timestamp, current_timestamp))

    }

}
fn convert_arc_mutex_to_vec(arc_mutex: Arc<Mutex<Vec<Post>>>) -> Vec<Post> {
    let mutex = Arc::try_unwrap(arc_mutex).expect("Failed to unwrap Arc");
    let inner = mutex.into_inner().expect("Failed to get inner value from Mutex");
    inner
}