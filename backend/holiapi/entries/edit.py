from holiapi.upload import UploadMsgs, MISSING_TAGS, MISSING_TITLE, save_upload_dict_as_json
from flask_restful import Resource, request
from flask_jwt_extended import jwt_required, current_user
from holiapi.utils import entries
from holiapi.logger import log
from holiapi.user import User
from holiapi import utils
from holiapi.entries import filter_tags

from holiapi.api_limiter import limiter

class EditEntry(Resource):
    decorators = [jwt_required(), limiter.limit("40/second")]
    def get(self):
        uid = request.args.get("uid")

        if not uid:
            return 400
        try:
            uid = int(uid)
        except:
            return 400

        # show usid if admin
        if current_user.is_admin():
            entry = entries[uid]
            entry["usid"] = entries.usid_dict[uid]
            return entry

        # has not uploaded this entry    
        if not uid in current_user.uploaded:
            return 400

        return entries[uid]

    decorators = [jwt_required(), limiter.limit("10/second")]
    def post(self):
        msg = UploadMsgs()

        uid = request.args.get("uid")

        if not uid:
            return 400
        try:
            uid = int(uid)
        except:
            return 400

        # has not uploaded this entry    
        if not (uid in current_user.uploaded) and not current_user.is_admin():
            return 400

        json_data = request.get_json(force=True)

        title = json_data["title"]

        # TODO: implement some way to update the date internally!!!!
        #returned_date = json_data["date"]
        tags = json_data["tags"]

        if title == "":
            msg.missing_title = MISSING_TITLE

        if tags == "":
            msg.missing_tags = MISSING_TAGS

        if msg.has_errors():
            return msg.as_json()

        entry = entries[uid]
        entry["title"] = title
        entry["tags"] = tags.split()
        
        save_upload_dict_as_json(entry, uid)
        log(f"{current_user.user_id}/{current_user.username}/{current_user.htl_class} edited entry uid={uid}, title=({title}), tags=({tags}).")
        
        msg.successful_upload = "Die Änderungen wurden abgespeichert!"
        return msg.as_json()
        

def get_editable_entries(user: User):
    if user.is_admin():
        return entries.entries

    uploaded_entry_ids = user.uploaded
    print(f"uploaded_entry_ids: {uploaded_entry_ids}")
    
    own_entries = {}
    
    for entry_id in uploaded_entry_ids:
        own_entries[entry_id] = entries[entry_id]

    return own_entries

class EditEntries(Resource):
    decorators = [jwt_required(), limiter.limit("40/second")]
    def get(self):
        own_entries = get_editable_entries(current_user)

        page = 0
        if request.args.get("page"):
            try:
                page = int(request.args.get("page"))
            except:
                return {}

        tags = request.args.get("tags")
        if tags:
            returned_tags = tags.split()
            print(returned_tags)
            own_entries = filter_tags.filter_for_tags(returned_tags, own_entries)
        else:
            tags = ""

        start, end, page_count = utils.limit_end_len(page, len(own_entries))
        if page > page_count or page < 0:
            return 400

        return {
            "entries": list(own_entries.values())[start:end],
            "page_count": page_count
        }
