import unittest
import pytest

from holiapi.utils import limit_end_len, get_upload_entries

class TestUtils(unittest.TestCase):
    def test_limit_end_len(self):
        start, end, times = limit_end_len(2, 67)
        assert start == 32
        assert end == 48
        assert times == 4

    def test_get_entries(self):
        entries = get_upload_entries([], entry_path="test_entries/")
        assert len(entries) == 3

        entry0 = entries[0]
        assert entry0["uid"] == 0
        assert entry0["title"] == "test0"
        assert entry0["tags"] == ["test", "26.10.2022", "ET", "pdf"]

    def test_get_entries_with_tags(self):
        entries = get_upload_entries(["mehr"], entry_path="test_entries/")
        assert len(entries) == 1

        entry1 = entries[1]
        assert entry1["uid"] == 1
        assert entry1["title"] == "test1"
        assert entry1["tags"] == ["test", "26.10.2022", "ET", "pdf", "mehr", "entries", "png"]
    
