import os
from datetime import datetime
from pathlib import Path

TAG = os.environ["TAG"]


def main():
    changelog = Path("CHANGELOG.md")

    with changelog.open() as f:
        lines = f.readlines()

    insert_index: int
    for i in range(len(lines