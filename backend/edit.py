from upload import UploadMsgs, MISSING_TAGS, MISSING_TITLE, save_upload_dict_as_json
import utils
import filter_tags
from flask_restful import Resource, request
from flask_jwt import jwt_required, current_identity
from utils import entries
from logger import log

from api_limiter import limiter

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

        # has not uploaded this entry    
        if not (uid in current_identity.id["uploaded"]):
            return 400

        global entries
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
        if not (uid in current_identity.id["uploaded"]):
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

        global entries
        entry = entries[uid]
        entry["title"] = title
        entry["tags"] = tags.split()
        
        save_upload_dict_as_json(entry, uid)
        log(f"{current_identity.id['user_id']}/{current_identity.id['username']}/{current_identity.id['htl_class']} edited entry uid={uid}, title=({title}), tags=({tags}).")
        
        msg.successful_upload = "Die Änderungen wurden abgespeichert!"
        return msg.as_json()
        


class EditEntries(Resource):
    decorators = [jwt_required(), limiter.limit("40/second")]
    def get(self):
        uploaded_entry_ids = current_identity.id["uploaded"]
        print(f"uploaded_entry_ids: {uploaded_entry_ids}")

        global entries
        own_entries = {}
        for entry_id in uploaded_entry_ids:
            own_entries[entry_id] = entries[entry_id]

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

        if page*16 >= len(entries):
            return {}
        start, end, page_count = utils.limit_end_len(page, len(entries))
        if page > page_count or page < 0:
            return 400

        return list(own_entries.values())[start:end]
