#!/usr/bin/python3
import sys

if __name__ == "__main__":
    html_path = sys.argv[1]
    with open(html_path) as f:
        html = f.read()

    css_path = "./scripts/hack.css"
    with open(css_path) as f:
        css = f.read()
    css = css.replace(" ", "")
    css = css.replace("\n", "")

    html = html.replace("</head>", f"<style>{css}</style></head>")

    with open(html_path, "w") as f:
        f.write(html)
