#!/usr/bin/env python3
import requests
from pathlib import Path

links = {
    "English": "https://en.wikipedia.org/wiki/Mars",
    "Chinese": "https://zh.wikipedia.org/wiki/%E7%81%AB%E6%98%9F",
    "Russian": "https://ru.wikipedia.org/wiki/%D0%9C%D0%B0%D1%80%D1%81",
    "French": "https://fr.wikipedia.org/wiki/Mars_(plan%C3%A8te)",
    "Arabic": "https://ar.wikipedia.org/wiki/%D8%A7%D9%84%D9%85%D8%B1%D9%8A%D8%AE",
    "Spanish": "https://es.wikipedia.org/wiki/Marte_(planeta)",
}

dataset_dir = Path("dataset/wikipedia_mars")
dataset_dir.mkdir(parents=True, exist_ok=True)

for tag, link in links.items():
    path = dataset_dir / f"{tag}.html"

    if path.exists():
        print(f"Skipping    {tag:<12} {link}")
    else:
        print(f"Downloading {tag:<12} {link}")
        html = requests.get(link).text
        with open(dataset_dir / f"{tag}.html", "w") as f:
            f.write(html)

print("Done")
