from flask_restful import Resource
from holiapi.utils import entries

def get_unique_tags(entries):
    tags = {}
    
    for entry in entries.values():
        for tag in entry["tags"]:
            # lowercase tags in search bar
            tag = tag.lower();
            if tag not in tags.keys():
                tags[tag] = 1
            else:
                tags[tag] += 1
    return tags

def get_unique_dict_tags(entries):
    tags = []
    unique_tags = get_unique_tags(entries)
    for (name, count) in unique_tags.items():
        tags.append({
            "name": name,
            "count": count
        })
    return tags

class UniqueTag:
    def __init__(self, name: str, count: int):
        self.name = name
        self.count = count

class UniqueTags(Resource):
    def get(self):
        unique_tags = get_unique_dict_tags(entries)
        return unique_tags
