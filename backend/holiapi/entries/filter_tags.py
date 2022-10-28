
def check_if_tags_found(tags, entry):
    for tag in tags:
        
        is_in_tags = False
        if tag.lower() in entry["title"].lower():
            is_in_tags = True

        for entry_tag in entry["tags"]:
            if tag.lower() == entry_tag.lower():
                is_in_tags = True
                break
        
        if not is_in_tags:
            return False
    return True

def filter_for_tags(tags, uploads):
    filtered_uploads = {}
    for entry in uploads.values():
        if not check_if_tags_found(tags, entry):
            continue
        filtered_uploads[entry["uid"]] = entry
        #filtered_uploads.append(entry)
    
    return filtered_uploads