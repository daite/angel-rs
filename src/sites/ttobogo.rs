use crate::*;

#[tokio::main]
pub async fn run(search_words: &str) -> Result<DataStatus, Box<dyn Error>> {
    let search_url = format!("{}{}", consts::TTOBOGO_SEARCH_URL, search_words);
    let data = get_data(&search_url).await?;
    if data.len() == 0 {
        println!("**** [TTOBOGO] NO TORRENT DATA ****");
        return Ok(DataStatus::NotFound)
    }
    let mut tasks = vec![];
    
    let r: Vec<(String, String)> = vec![];
    let result = Arc::new(Mutex::new(r));

    for d in data {
        let result = Arc::clone(&result);
        tasks.push(tokio::spawn(async move {
           let magnet = get_magnet(&d.1).await.unwrap();
           if magnet == "NO MAGNET" {
               return
           }
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
    Ok(DataStatus::Found)
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
    if let Some(node) = doc.find(Attr("class", "btn btn-blue")).next() {
        let title = node.attr("onclick").unwrap();
        let cap = re.captures(title).unwrap();
        let magnet = format!("{}{}", magnet_prefix, &cap[0]);
        return Ok(magnet)
    }
    Ok(String::from("NO MAGNET"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_magnet_function() {
        let data = vec![
        ("https://ttobogo.net/post/160049", 
        "magnet:?xt=urn:btih:d77a44e97d82ee818f017a3f7cf0dc6c5e625357"),
        ("https://ttobogo.net/post/181711",
        "magnet:?xt=urn:btih:2039d8aebb9f406cfc114c909982d36460b65639"),
        ("https://ttobogo.net/post/178001",
        "magnet:?xt=urn:btih:000e523427aa08e249058fb90f230fe92e9e3adc"),
        ("https://ttobogo.net/post/170793",
        "magnet:?xt=urn:btih:c2f7e50f853f3d3aacd585d00fc1c3e64c123153"),
        ];
        for d in data {
            assert_eq!(tokio_test::block_on(get_magnet(d.0)).unwrap(), d.1);
        }
    }
    #[test]
    fn test_get_data_function() {
        let search_url = format!("{}{}", consts::TTOBOGO_SEARCH_URL, "시즌2.E141");
        let data = vec![
            ("어서와 한국은 처음이지 시즌2.E141.210114.720p-NEXT".to_owned(), 
            "https://ttobogo.net/post/181711".to_owned()),
            ("살림하는 남자들 시즌2.E141.200219.720p-NEXT".to_owned(),
            "https://ttobogo.net/post/17920".to_owned()),
        ];
        let result = tokio_test::block_on(get_data(&search_url)).unwrap();
        for n in 0..data.len() {
            assert_eq!(data[n], result[n]);
        }
    }
}