use crate::*;

#[tokio::main]
pub async fn run(search_words: &str) -> Result<(), Box<dyn Error>> {
    let search_url = format!("{}{}", consts::TSHARE_SEARCH_URL, search_words);
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
    for node in doc.find(Attr("class", "list-item-row")) {
        let title = node.find(Attr("class", "matter"))
                    .next()
                    .unwrap()
                    .find(Name("p"))
                    .next()
                    .unwrap()
                    .text().to_owned();
       let link = node.find(Name("a"))
                    .next()
                    .unwrap()
                    .attr("href")
                    .unwrap().to_owned();
        data.push((title, link));
    }
    Ok(data)
}

pub async fn get_magnet(bbs_url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client.get(bbs_url)
              .header(USER_AGENT, consts::MY_USER_AGENT)
              .send()
              .await?;
    let body = res.text().await?;
    let doc = Document::from(&body[..]);
    let magnet = doc.find(Name("td"))
                .skip(1)
                .next()
                .unwrap()
                .find(Name("a"))
                .next()
                .unwrap()
                .attr("href")
                .unwrap().to_owned();
   Ok(magnet)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_magnet_function() {
        let data = vec![
        ("https://tshare.org/entertainment/13062", 
        "magnet:?xt=urn:btih:6bb34701c93505114029e5c91a0e88a30c11703b"),
        ("https://tshare.org/entertainment/12870",
        "magnet:?xt=urn:btih:ecf2f09ab88bfbf3c1f8e8dac02faff638433d23"),
        ("https://tshare.org/entertainment/12366",
        "magnet:?xt=urn:btih:d77a44e97d82ee818f017a3f7cf0dc6c5e625357"),
        ];
        for d in data {
            assert_eq!(tokio_test::block_on(get_magnet(d.0)).unwrap(), d.1);
        }
    }
    #[test]
    fn test_get_data_function() {
        let search_url = format!("{}{}", consts::TSHARE_SEARCH_URL, "E177.201228.720p");
        let data = vec![
            ("동상이몽2 너는 내운명.E177.201228.720p-NEXT.mp4".to_owned(), 
            "https://tshare.org/entertainment/13065".to_owned()),
            ("동상이몽2 너는 내운명.E177.201228.720p-NEXT".to_owned(),
            "https://tshare.org/entertainment/13062".to_owned()),
        ];
        let result = tokio_test::block_on(get_data(&search_url)).unwrap();
        for n in 0..data.len() {
            assert_eq!(data[n], result[n]);
        }
    }
}