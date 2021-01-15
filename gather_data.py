#!/usr/bin/env python3

import requests 
from bs4 import BeautifulSoup as BS
from http.client import HTTPConnection
from functools import wraps
import pprint

headers = {
    "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 11_1_0)",
}

def debug(func):
    @wraps(func)
    def wrapper(*args, **kwds):
        HTTPConnection.debuglevel = 1
        return func(*args, **kwds)
    return wrapper

def get_tags(url, find_tag, tag_class_name):
    r = requests.get(url, headers=headers)
    soup = BS(r.content, "lxml")
    tags = soup.find_all(find_tag, {'class': tag_class_name})
    return tags

def get_bbs_urls(search_url):
    bbs_urls = []
    tags = get_tags(search_url, 'a', "subject")
    for tag in tags:
        bbs_urls.append(tag["href"])
    return bbs_urls

@debug
def get_magnet(bbs_url):
    tag = get_tags(bbs_url, 'a', "btn btn-blue")
    magnet = tag[0]["onclick"].split("'")[1]
    return magnet


if __name__ == "__main__":
    search_url = "https://ttobogo.net/search?skeyword=%EC%96%B4%EC%84%9C%EC%99%80+%ED%95%9C%EA%B5%AD%EC%9D%80+%EC%B2%98%EC%9D%8C%EC%9D%B4%EC%A7%80"
    for bbs_url in get_bbs_urls(search_url):
        fm = '("' + bbs_url  + '",' + "\n" + '"' + get_magnet(bbs_url) + '"),'
        print(fm)
