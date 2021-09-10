import os
from datetime import datetime
from pathlib import Path

TAG = os.environ["TAG"]


def main():
    changelog = Path("CHANGELOG.md")

    with changelog.open() as f:
        lines = f.readlines()

    insert_index: int
    for i in range(len(lines)):
        line = lines[i]
        if line.startswith("## Unreleased"):
            insert_index = i + 1
        elif line.startswith(f"## [{TAG}]"):
            print("CHANGELOG already up-to-date")
            return
        elif line.startswith("## [v"):
            br