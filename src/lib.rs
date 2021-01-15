#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, format};
use reqwest;
use reqwest::header::USER_AGENT;
use select::document::Document;
use select::predicate::{Attr};
use std::error::Error;
use std::sync::{Mutex, Arc};
use regex::Regex;

mod consts;

pub fn print_table(p: Vec<(String, String)>) {
    let mut table = Table::new();
    table.set_titles(row!["Title", "Magnet"]);
    for zd in p {
        table.add_row(Row::new(vec![
             Cell::new(&zd.0),
             Cell::new(&zd.1),
        ]));
      }
      table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
      table.printstd();
}

pub mod ttobogo {
    use super::*;
    #[tokio::main]
    pub async fn run(search_words: &str) -> Result<(), Box<dyn Error>> {
        let search_url = format!("{}{}", consts::TTOBOGO_SEARCH_URL, search_words);
        let data = get_data(&search_url).await?;
        let mut tasks = vec![];
        
        let r: Vec<(String, String)> = vec![];
        let result = Arc::new(Mutex::new(r));
    
        for d in data {
            let result = Arc::clone(&result);
            tasks.push(tokio::spawn(async move {
               let magnet = get_magnet(&d.1).await.unwrap();
               let mut r = result.lock().unwrap();
               (*r).push((d.0, magnet));
            }));
        }
        for task in tasks {
            task.await?;
        }
        let p = &mut *result.lock().unwrap();
        p.sort();
        p.reverse();
        print_table(p.to_vec());
        Ok(())
    }
    
    pub async fn get_data(search_url: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
        let client  = reqwest::Client::new();
        let res = client.get(search_url)
                .header(USER_AGENT, consts::MY_USER_AGENT)
                .send()
                .await?;
        let body = res.text().await?;
        let doc = Document::from(&body[..]);
        let mut data: Vec<(String, String)> = vec![];
        for node in doc.find(Attr("class", "subject")) {
            let title = node.attr("title").unwrap().to_owned();
            let link = node.attr("href").unwrap().to_owned();
            data.push((title, link));
        }
        Ok(data)
    }
    
    pub async fn get_magnet(bbs_url: &str) -> Result<String, Box<dyn Error>> {
        let magnet_prefix = "magnet:?xt=urn:btih:";
        let re = Regex::new(r"[0-9a-z]{40}").unwrap();
        let client = reqwest::Client::new();
        let res = client.get(bbs_url)
                  .header(USER_AGENT, consts::MY_USER_AGENT)
                  .send()
                  .await?;
        let body = res.text().await?;
        let doc = Document::from(&body[..]);
        let title = doc.find(Attr("class", "btn btn-blue"))
                   .next().unwrap()
                   .attr("onclick").unwrap();
        let cap = re.captures(title).unwrap();
        let magnet = format!("{}{}", magnet_prefix, &cap[0]);
        Ok(magnet)
    }
}

#[tokio::test]
async fn test_get_magnet_function() {
    let data = vec![
        ("https://ttobogo.net/post/160049", 
        "magnet:?xt=urn:btih:d77a44e97d82ee818f017a3f7cf0dc6c5e625357"),
        ("https://ttobogo.net/post/181711",
        "magnet:?xt=urn:btih:2039d8aebb9f406cfc114c909982d36460b65639"),
        ("https://ttobogo.net/post/178001",
        "magnet:?xt=urn:btih:000e523427aa08e249058fb90f230fe92e9e3adc"),
        ("https://ttobogo.net/post/174431",
        "magnet:?xt=urn:btih:0b5cc6e2e8fe6fad51c790f981f749d40ea7abdf"),
        ("https://ttobogo.net/post/174199",
        "magnet:?xt=urn:btih:0b5cc6e2e8fe6fad51c790f981f749d40ea7abdf"),
        ("https://ttobogo.net/post/170793",
        "magnet:?xt=urn:btih:c2f7e50f853f3d3aacd585d00fc1c3e64c123153"),
        ("https://ttobogo.net/post/170794",
        "magnet:?xt=urn:btih:5e074ce2c9926f2c5b58d316cf0ad777fdad31cc"),
        ("https://ttobogo.net/post/170781",
        "magnet:?xt=urn:btih:c2f7e50f853f3d3aacd585d00fc1c3e64c123153"),
        ("https://ttobogo.net/post/166650",
        "magnet:?xt=urn:btih:105e1e8640008f03330a6fc9849c0b820b43f335"),
        ("https://ttobogo.net/post/166647",
        "magnet:?xt=urn:btih:105e1e8640008f03330a6fc9849c0b820b43f335"),
        ("https://ttobogo.net/post/166625",
        "magnet:?xt=urn:btih:bea7921e8491e6ab0469d6a14e6915f5dcae91a7"),
        ("https://ttobogo.net/post/162111",
        "magnet:?xt=urn:btih:4836fe3a4bc8b96022d6224ffc11ce9140342a56"),
        ("https://ttobogo.net/post/162105",
        "magnet:?xt=urn:btih:a0fa6f65e8d2518fb045f543ee77626069832c40"),
        ("https://ttobogo.net/post/162103",
        "magnet:?xt=urn:btih:4836fe3a4bc8b96022d6224ffc11ce9140342a56"),
        ("https://ttobogo.net/post/157711",
        "magnet:?xt=urn:btih:0512dc350ecca55805e873ce6c16e5bdf7de3171"),
        ("https://ttobogo.net/post/157419",
        "magnet:?xt=urn:btih:0512dc350ecca55805e873ce6c16e5bdf7de3171"),
        ("https://ttobogo.net/post/157196",
        "magnet:?xt=urn:btih:103961cd3dd19ed321f6fa28b57577945bef12c7"),
        ("https://ttobogo.net/post/157147",
        "magnet:?xt=urn:btih:b275f3e4d53dc493ddb884b933564aab510b9214"),
        ("https://ttobogo.net/post/157150",
        "magnet:?xt=urn:btih:b275f3e4d53dc493ddb884b933564aab510b9214"),
        ("https://ttobogo.net/post/153078",
        "magnet:?xt=urn:btih:721e56f0fa492539690c597a78e92bd2dcb71539"),
        ("https://ttobogo.net/post/153021",
        "magnet:?xt=urn:btih:721e56f0fa492539690c597a78e92bd2dcb71539"),
        ("https://ttobogo.net/post/152930",
        "magnet:?xt=urn:btih:ea960fd7ae7eb3038f46cd06229a58923836e77d"),
        ("https://ttobogo.net/post/152836",
        "magnet:?xt=urn:btih:721e56f0fa492539690c597a78e92bd2dcb71539"),
        ("https://ttobogo.net/post/152804",
        "magnet:?xt=urn:btih:764f5f667dcc8fb5179c025b2987925dbdfe56bd"),
        ("https://ttobogo.net/post/152803",
        "magnet:?xt=urn:btih:764f5f667dcc8fb5179c025b2987925dbdfe56bd"),
        ("https://ttobogo.net/post/147990",
        "magnet:?xt=urn:btih:d93f97e03f5d52ba8022993dfb72b182f1fe3b98"),
    ];
    for d in data {
        assert_eq!(&ttobogo::get_magnet(d.0).await.unwrap()[..], d.1);
    }
}

#[tokio::test]
async fn test_get_data_function() {
    let search_words = "https://ttobogo.net/search?skeyword=%EC%8B%9C%EC%A6%8C2.E141";
    let data = vec![
        ("어서와 한국은 처음이지 시즌2.E141.210114.720p-NEXT".to_owned(), 
        "https://ttobogo.net/post/181711".to_owned()),
        ("살림하는 남자들 시즌2.E141.200219.720p-NEXT".to_owned(),
        "https://ttobogo.net/post/17920".to_owned()),
    ];
    let result = ttobogo::get_data(search_words).await.unwrap();
    for n in 0..data.len() {
        assert_eq!(data[n], result[n]);
    }
}
