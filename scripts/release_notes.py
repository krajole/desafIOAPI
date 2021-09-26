# encoding: utf-8

"""
Prepares markdown release notes for GitHub releases.
"""

import os
from typing import List, Optional

import packaging.version

TAG = os.environ["TAG"