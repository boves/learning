# web_scraper.py
# URL = "https://realpython.com/python-web-scraping-practical-introduction/"

from urllib.request import urlopen

url = "http://olympus.realpython.org/profiles/aphrodite"
page = urlopen(url)

html_bytes = page.read()
html = html_bytes.decode("utf-8")
print(html)

title_index = html.find("<title>")
start_index = title_index + len("<title>")
print(start_index)
end_index = html.find("</title>")
print(end_index)

title = html[start_index:end_index]
print(title)

print("this next example won't work well without regex")
url = "http://olympus.realpython.org/profiles/poseidon"
page = urlopen(url)
html = page.read().decode("utf-8")
start_index = html.find("<title>") + len("<title>")
end_index = html.find("</title>")
title = html[start_index:end_index]
print(title)

import re

print(re.findall("ab*c", "ac"))
print(re.findall("ab*c", "abcd"))
print(re.findall("ab*c", "abcac"))
print(re.findall("ab*c", "abdc"))

print(re.findall("ab*c", "ABC"))
print(re.findall("ab*c", "ABC", re.IGNORECASE))
print(re.findall("a.c", "abc"))
print(re.findall("a.c", "abbc"))
print(re.findall("a.c", "ac"))
print(re.findall("a.c", "acc"))

print(re.findall("a.*c", "abc"))
print(re.findall("a.*c", "abbc"))
print(re.findall("a.*c", "ac"))
print(re.findall("a.*c", "acc"))

match_results = re.search("ab*c", "ABC", re.IGNORECASE)
print(match_results.group())
string = "Everything is <replaced> if it's in <tags>."
string = re.sub("<.*?>", "ELEPHANTS", string) # .*? is non-greedy
print(string)

url = "http://olympus.realpython.org/profiles/dionysus"
page = urlopen(url)
html = page.read().decode("utf-8")

pattern = "<title.*?>.*?</title.*?>"
match_results = re.search(pattern, html, re.IGNORECASE)
title = match_results.group()
title = re.sub("<.*?>", "", title) # Remove HTML tags
print(title)

print("Check Your Understanding")
print("1. Write a program that grabs the full HTML from the given URL")
url = "http://olympus.realpython.org/profiles/dionysus"
html_page = urlopen(url)
html_text = html_page.read().decode("utf-8")

for string in ["Name:", "Favorite Color:"]:
	string_start_idx = html_text.find(string)
	text_start_idx = string_start_idx + len(string)

	next_html_tag_offset = html_text[text_start_idx:].find("<")
	text_end_idx = text_start_idx + next_html_tag_offset

	raw_text = html_text[text_start_idx : text_end_idx]
	clean_text = raw_text.strip(" \r\n\t")
	print(clean_text)

print("Using BeautifulSoup Objects")
print("***************************")

from bs4 import BeautifulSoup
from urllib.request import urlopen

url = "http://olympus.realpython.org/profiles/dionysus"
page = urlopen(url)
html = page.read().decode("utf-8")
soup = BeautifulSoup(html, "html.parser")

print(soup.get_text())

image1, image2 = soup.find_all("img")
print(image1.name)
print(image2.name)

print(image1["src"])
print(image2["src"])

print(soup.title)
print(soup.title.string)
print(soup.find_all("img", src="/static/dionysus.jpg"))

print("Check Your Understanding")
print("2. Use BeautifulSoup to parse a webpage")
from urllib.request import urlopen
from bs4 import BeautifulSoup

base_url = "http://olympus.realpython.org"
html_page = urlopen(base_url + "/profiles")
html_text = html_page.read().decode("utf-8")

soup = BeautifulSoup(html_text, "html.parser")

for link in soup.find_all("a"):
	link_url = base_url + link["href"]
	print(link_url)


print("Interact with HTML forms")
import mechanicalsoup
browser = mechanicalsoup.Browser()

url = "http://olympus.realpython.org/login"
page = browser.get(url)

print(page.soup)

print("Submit a form with MechanicalSoup")
# 1 
browser = mechanicalsoup.Browser()
url = "http://olympus.realpython.org/login"
login_page = browser.get(url)
login_html = login_page.soup

# 2
form = login_html.select("form")[0]
form.select("input")[0]["value"] = "zeus"
form.select("input")[1]["value"] = "ThunderDude"

# 3
profiles_page = browser.submit(form, login_page.url)

print(profiles_page.url)
links = profiles_page.soup.select("a")

for link in links:
		address = link["href"]
		text = link.text
		print(f"{text}: {address}")

base_url = "http://olympus.realpython.org"
for link in links:
	address = base_url + link["href"]
	text = link.text
	print(f"{text}: {address}")

browser = mechanicalsoup.Browser()
page = browser.get("http://olympus.realpython.org/dice")
tag = page.soup.select("#result")[0]
result = tag.text

print(f"The result of your dice roll is: {result}")

import time
for i in range(4):
	page = browser.get("http://olympus.realpython.org/dice")
	tag = page.soup.select("#result")[0]
	result = tag.text 
	print(f"The result of your dice roll is: {result}")




