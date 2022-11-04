//!
//! Simple CLI app that can print xkcd comics IN the terminal.
//!
use anyhow::Result;
use rand::Rng;
use reqwest;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use tokio;
use viuer::Config;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comic {
    pub month: String,
    pub num: i64,
    pub link: String,
    pub year: String,
    pub news: String,
    #[serde(rename = "safe_title")]
    pub safe_title: String,
    pub transcript: String,
    pub alt: String,
    pub img: String,
    pub title: String,
    pub day: String,
}

// Wrapping a get request from the `reqwest` crate...
async fn simple_get(s: &str) -> Result<String> {
    Ok(reqwest::get(s).await?.text().await?)
}

// Download comic images to disk.
async fn download_file(s: &str, name: &str) -> Result<()> {
    let resp = reqwest::get(s).await?;

    let mut file = File::create(&name)?;
    file.write_all(&resp.bytes().await?)?;

    Ok(())
}

// Use viuer to print images into a terminal.
fn print_image(f: &str) -> Result<()> {
    std::process::Command::new("clear").status()?; // clear the terminal first.
    let img = image::open(f)?;

    let conf = Config {
        x: 5,
        y: 5,
        width: Some(80),
        height: Some(25),
        ..Default::default()
    };
    viuer::print(&img, &conf)?;
    Ok(())
}

// Gets a random comic for you :)
async fn random_comic() -> Result<()> {
    let mut rng = rand::thread_rng();

    let target = format!("https://xkcd.com/{}/info.0.json", rng.gen_range(1..2693)); //Latest count
                                                                                     //was 2693 according to https://www.explainxkcd.com/wiki/index.php/List_of_all_comics
    let res = simple_get(&target).await?;
    if let Ok(res) = serde_json::from_str::<Comic>(&res) {
        let name = format!("data/{}", res.img.clone().split('/').last().unwrap());
        download_file(&res.img, &name).await?;
        print_image(&name)?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    random_comic().await
}
