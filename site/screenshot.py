# /// script
# requires-python = ">=3.10"
# dependencies = ["playwright"]
# ///

from playwright.sync_api import sync_playwright

with sync_playwright() as p:
    browser = p.chromium.launch()
    page = browser.new_page(viewport={"width": 1280, "height": 640})
    page.goto("file:///Users/njha/Development/github.com/nikhiljha/cargo-delenda/site/social-card.html")
    page.screenshot(path="/Users/njha/Development/github.com/nikhiljha/cargo-delenda/site/social-card.png", clip={"x": 0, "y": 0, "width": 1280, "height": 640})
    browser.close()
