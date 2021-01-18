use crate::*;

#[tokio::main]
pub async fn run(search_words: &str) -> Result<DataStatus, Box<dyn Error>> {
    let search_url = format!("{}{}", consts::TORRENTVIEW_SEARCH_URL, search_words);
    let data = get_data(&search_url).await?;
    if data.len() == 0 {
        println!("**** [TORRENTVIEW] NO TORRENT DATA ****");
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
    for node in doc.find(Attr("class", "media-heading")) {
        let title = node.find(Name("a"))
                    .next()
                    .unwrap()
                    .text()
                    .trim().to_owned();
       let link = node.find(Name("a"))
                    .next()
                    .unwrap()
                    .attr("href")
                    .unwrap();
        let s = Url::parse(consts::TORRENTVIEW_SEARCH_URL).unwrap();
        let absolute_link = s.join(link).unwrap().as_str().to_owned();
        data.push((title, absolute_link));
    }
    Ok(data)
}

pub async fn get_magnet(bbs_url: &str)  -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client.get(bbs_url)
              .header(USER_AGENT, consts::MY_USER_AGENT)
              .send()
              .await?;
    let body = res.text().await?;
    let doc = Document::from(&body[..]);
    if let Some(node) = doc.find(Attr("class", "list-group")).next() {
        let result = node.text()
                    .trim()
                    .to_owned();
        return Ok(result)
    }     
    Ok(String::from("NO MAGNET"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_magnet_function() {
        let data = vec![
        ("https://torrentview28.com/bbs/board.php?bo_table=mov&wr_id=17806", 
        "magnet:?xt=urn:btih:c36bd92c970cadace937a0d22f0440384ca6b6da"),
        ("https://torrentview28.com/bbs/board.php?bo_table=mov&wr_id=17742",
        "magnet:?xt=urn:btih:7f45e15f60b295b1f42f1d5160409f0e1006dcb0"),
        ("https://torrentview28.com/bbs/board.php?bo_table=mov&wr_id=17725",
        "magnet:?xt=urn:btih:baeffe526ecb61e2e774b2e460a5bdddf3f1e195"),
        ];
        for d in data {
            assert_eq!(tokio_test::block_on(get_magnet(d.0)).unwrap(), d.1);
        }
    }
    #[test]
    fn test_get_data_function() {
        let search_url = format!("{}{}", consts::TORRENTVIEW_SEARCH_URL, "시즌2.E141");
        let data = vec![
            ("어서와 한국은 처음이지 시즌2.E141.210114.720p-NEXT".to_owned(), 
            "https://torrentview28.com/bbs/board.php?bo_table=enter&wr_id=21502".to_owned()),
            ("살림하는 남자들 시즌2.E141.200219.720p-NEXT".to_owned(),
            "https://torrentview28.com/bbs/board.php?bo_table=enter&wr_id=10402".to_owned()),
            ("살림하는 남자들 시즌2.E141.200219.720p-NEXT".to_owned(),
            "https://torrentview28.com/bbs/board.php?bo_table=enter&wr_id=10396".to_owned()),
        ];
        let result = tokio_test::block_on(get_data(&search_url)).unwrap();
        for n in 0..data.len() {
            assert_eq!(data[n], result[n]);
        }
    }
}
