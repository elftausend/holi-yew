import os
from typing import Tuple
import json
from holiapi.entries.filter_tags import check_if_tags_found
import re
from holiapi.config import PATH

def sorting(entry):
    # entry[1]: because an entry is a tuple with (hash, <values>)
    splitup = entry[1]["date"].split('.')
    return (splitup[2], splitup[1], splitup[0])

def limit_end_len(page: int, max_len: int) -> Tuple[int, int, int]:
    start = page*16
    end = (page+1) * 16

    if end > max_len:
        end = max_len

    times = int(max_len / 16)
    return (start, end, times)

def get_upload_entries(lookup_tags, user="admin", entry_path = f"{PATH}/static/uploaded/"):
    entries = os.listdir(entry_path)
    files_data = {}
    
    for entry in entries:
        with open(f"{entry_path}{entry}", mode="r") as file:
            upload = json.load(file)

            if upload["usid"] == user or user == "admin":
                if check_if_tags_found(lookup_tags, upload):
                    files_data[upload["uid"]] = upload
        
    #return dict(sorted(files_data.items(), key=sorting, reverse=True))
    return files_data

entries = get_upload_entries([])

def check_date(today, returned_date):
    if len(returned_date) == 0:
        return (today, "")

    x = re.search("^([0-2][0-9]|(3)[0-1])(\.)(((0)[0-9])|((1)[0-2]))(\.)\d{4}$", returned_date)
    
    if x == None:
        return (today, "Das eingegebene Datum ist inkorrekt.")
        
    today = str(returned_date[0:2]) + "." + str(returned_date[3:5]) + "." + str(returned_date[6:10])
    return (today, "")


def get_proglogo_from_file_type(file_ext: str):
    logo_path = None
    if file_ext == "rs":
        logo_path = "logos/prog_lang_logos/rust_logo.png"
    elif file_ext == "py":
        logo_path = "logos/prog_lang_logos/python_logo.png"
    elif file_ext == "js":
        logo_path = "logos/prog_lang_logos/javascript_logo.png"
    elif file_ext == "cpp":
        logo_path = "logos/prog_lang_logos/c_plus_plus_logo.jpeg"
    elif file_ext == "java":
        logo_path = "logos/prog_lang_logos/java_logo.jpeg"
    elif file_ext == "c":
        logo_path = "logos/prog_lang_logos/c_logo.png"
    return logo_path


def file_contents(filename):
    """ Given a filename,
        return the contents of that file
    """
    try:
        with open(f"{PATH}/{filename}", 'r') as f:
            # It's assumed our file contains a single line,
            # with our API key
            return f.read().strip()
    except FileNotFoundError:
        print(f"{filename} file not found")