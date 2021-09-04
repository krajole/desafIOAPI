import os
from datetime import datetime
from pathlib import Path

TAG = os.environ["TAG"]


def main():
    changelog = Path("CHANGELOG.md")

    with cha