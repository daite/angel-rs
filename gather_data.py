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

class TTobogo:

    def __init__(self, search_words):
        self.search_url =  "https://ttobogo.net/search?skeyword={}"\
            .format(search_words)
    
    def get_tags(self, url, find_tag, tag_class_name):
        r = requests.get(url, headers=headers)
        soup = BS(r.content, "lxml")
        tags = soup.find_all(find_tag, {'class': tag_class_name})
        return tags
    
    def get_bbs_urls(self):
        bbs_urls = []
        tags = self.get_tags(self.search_url, 'a', "subject")
        for tag in tags:
            bbs_urls.append(tag["href"])
        return bbs_urls

    def get_magnet(self, bbs_url):
        tag = self.get_tags(bbs_url, 'a', "btn btn-blue")
        magnet = tag[0]["onclick"].split("'")[1]
        return magnet

def gather_ttobogo_data():
    keyword = "어서와+한국은+처음이지"
    t = TTobogo(keyword)
    for bbs_url in t.get_bbs_urls():
        fm = '("' + bbs_url  + '",' + "\n" + '"' \
            + t.get_magnet(bbs_url) + '"),'
        print(fm)


class Torrentdia:

    def __init__(self, search_words):
        self.search_url =  "https://www.torrentdia.com/bbs/search.php?stx={}"\
            .format(search_words)
    
    def get_tags(self, url, find_tag, tag_class_name):
        r = requests.get(url, headers=headers)
        soup = BS(r.content, "lxml")
        tags = soup.find_all(find_tag, {'class': tag_class_name})
        return tags
    
    def get_bbs_urls(self):
        bbs_urls = []
        tags = self.get_tags(self.search_url, 'td', "list-subject web-subject")
        for tag in tags:
            bbs_url = requests.compat.urljoin(self.search_url, \
            tag.find("a", href=True)["href"])
            bbs_urls.append(bbs_url)
        return bbs_urls
