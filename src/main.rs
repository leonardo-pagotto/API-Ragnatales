use std::{fmt::format, fs::File, io::Write, thread::{self, Thread}, time::Duration};

use directories::UserDirs;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use models::MarketItem;

use crate::external::send_request;

pub mod external;
pub mod models;

#[tokio::main]
async fn main() {
    
    let mut current_page : i16 = 0;
    let mut result : Vec<MarketItem> = vec![];
    let directories = match UserDirs::new() {
        Some(dirs) => dirs,
        None => panic!("Não foi possivel acessar os diretorios do sistema operacional...")
    };
    
    let fist_contact = send_request(current_page).await;

    println!("🔎 Total de paginas à serem percorridas: {}", fist_contact.total_pages);
    println!("🔄 Iniciando processo em 3 segundos...");
    println!(" ");

    thread::sleep(Duration::from_secs(3));

    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}",
    ).unwrap().progress_chars("##-");

    let pb = ProgressBar::new(fist_contact.total_pages as u64);
    pb.set_style(sty.clone());

    while current_page < fist_contact.total_pages {
        let res = send_request(current_page).await;
        result.extend(res.rows);
        
        pb.inc(1);
        current_page += 1;
    }
    
    if let Some(dir) = directories.document_dir() {
        let path = dir.join("big_json.json");
        let json_str = serde_json::to_string_pretty(&result).unwrap();

        let mut file = File::create(path).unwrap();
        file.write_all(json_str.as_bytes()).unwrap();
    }

    println!(" ");

    println!("✅ Processo concluido!")
}