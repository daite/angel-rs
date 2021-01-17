use crate::*;

pub fn run (search_words: &str) -> Result<(), Box<dyn Error>> {
    let text = get_body(consts::TORRENTUBE_SEARCH_URL, search_words);
    let titles = get_titles(&text);
    let magnets = get_magnets(&text);
    let zipped_data: Vec<(String, String)> = titles.into_iter()
                            .zip(magnets.into_iter())
                            .collect();
    print_table(zipped_data);
    Ok(())
}

fn get_body(basic_url: &str, search_words: &str) -> String {
    let search_url = format!("{}{}", basic_url, search_words);
    let res = reqwest::blocking::get(&search_url[..]).unwrap();
    let body = res.text().unwrap();
    body
}

fn get_titles(text: &str) -> Vec<String> { 
    let re = Regex::new(r"'fn': '(.+?)'").unwrap();    
    let mut v: Vec<String> = vec![];
    let result = re.captures_iter(text);
    for r in result {
      let data = r.get(1).unwrap();
      v.push(data.as_str().to_string());
    }
   v
}

fn get_magnets(text: &str) -> Vec<String> { 
    let magnet_prefix = "magnet:?xt=urn:btih:";
    let re = Regex::new(r"'hs': '([0-9,a-z]{40})'").unwrap();    
    let mut v: Vec<String> = vec![];
    let result = re.captures_iter(text);
    for r in result {
      let data = r.get(1).unwrap();
      v.push(format!("{}{}", magnet_prefix, data.as_str()));
    }
   v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_magnet_function() {
        let search_item = "동상이몽2 너는 내운명 E149 200608";
        let text = get_body(consts::TORRENTUBE_SEARCH_URL, search_item);
        let titles = get_titles(&text);
        assert_eq!(titles, vec![
            "동상이몽2 너는 내운명.E149.200608.720p-NEXT.mp4 [안깨지는영상]",
            "동상이몽2 너는 내운명.E149.200608.720p-NEXT (안깨지는영상)",
            "동상이몽2 너는 내운명.E149.200608.720p-NEXT",
        ])
    }
    #[test]
    fn test_get_title_function() {
        let search_item = "동상이몽2 너는 내운명 E149 200608";
        let text = get_body(consts::TORRENTUBE_SEARCH_URL, search_item);
        let magnets = get_magnets(&text);
        assert_eq!(magnets, vec![
            "magnet:?xt=urn:btih:7226f232a460e74e0a41aa355e9b005298381d3b",
            "magnet:?xt=urn:btih:a87ab1393d8f856be8fe3cc50192cadc9cc4acde",
            "magnet:?xt=urn:btih:3818bc5431b7390eaeabc1e9877384e6a80cdeda",
        ])
    }
}

