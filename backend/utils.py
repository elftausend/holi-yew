import os
from typing import Tuple
import json
from pathlib import Path
from filter_tags import check_if_tags_found

dir_path = os.path.dirname(os.path.realpath(__file__))

def sorting(entry):
    splitup = entry["date"].split('.')
    return (splitup[2], splitup[1], splitup[0])

def limit_end_len(page: int, max_len: int) -> Tuple[int, int, int]:
    start = page*16
    end = (page+1) * 16

    if end > max_len:
        end = max_len

    times = int(max_len / 16)
    return (start, end, times)

def get_upload_entries(lookup_tags, user="admin"):
    entries = Path(f"{dir_path}/static/uploaded/").rglob('*.json')
    files_data = []
    
    for entry in entries:
        with open(entry, mode="r") as file:
            x = json.load(file)

            if x["uploader"] == user or user == "admin":

                if check_if_tags_found(lookup_tags, entry):
                    files_data.append(x)
        
    return sorted(files_data, key=sorting, reverse=True)