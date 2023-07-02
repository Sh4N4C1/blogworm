use clap::{App, Arg};
use std::process;
use blogworm::{POSTSRC_LIST, Summary};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::error::Error;
use std::io::{self, Read, Write};
use std::fs::{self, File};

pub struct Config{
    save_path: String,
}

pub fn get_configs() -> Result<Config, Box<dyn Error>>{
    let user_home_dir =  dirs::home_dir().expect("Fail to get home dir");
    let mut default_save_path = PathBuf::from(user_home_dir);
    default_save_path.push(".blogworm/new_post.json");

    let app = App::new("Blogworm").version("1.0")
        .author("shanacl").about("Just get at latest blog tool ~")
        .arg(Arg::with_name("save").short("s").long("save").value_name("save").help("new post list save path").takes_value(true).required(false).default_value(default_save_path.to_str().unwrap()),)
        .arg(Arg::with_name("show").short("w").long("show").value_name("show").help("show all poosrt src").takes_value(false).required(false));

    if app.clone().get_matches().is_present("show"){
        for postsrc in POSTSRC_LIST.iter(){
            println!("[*] {}PostSrc: {}", crate::CLIP, postsrc.summarize());
        }
        process::exit(1);
    }

    let args =  app.clone().get_matches();
    let save_path = args.value_of("save").unwrap();


    println!("[*] Post list will save AT: {}", save_path);

    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get current timestamp")
        .as_secs();

    let last_run_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get current timestamp")
        .as_secs();
    Ok(Config {save_path: save_path.to_string()})

}
impl Config {
    pub fn get_save_path(&self) -> &str{
        &self.save_path
    }
}
