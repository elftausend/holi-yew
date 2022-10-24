import os
import utils
import json
import filter_tags
from flask_restful import Resource, request
from flask_jwt import jwt_required, current_identity
from utils import entries

PATH = os.path.dirname(os.path.realpath(__file__))

from api_limiter import limiter

class Entries(Resource):
    #@jwt_required()
    decorators = [jwt_required(), limiter.limit("40/second")]
    def get(self):

        global entries
        local_entries = entries

        page = 0
        if request.args.get("page"):
            try:
                page = int(request.args.get("page"))
            except:
                pass

        tags = request.args.get("tags")
        if tags:
            returned_tags = tags.split()
            print(returned_tags)
            local_entries = filter_tags.filter_for_tags(returned_tags, entries)
        else:
            tags = ""

        if page*16 >= len(entries):
            return {}
        start, end, page_count = utils.limit_end_len(page, len(entries))
        if page > page_count or page < 0:
            return 400
        
        # return page count as well
        return list(local_entries.values())[start:end]

class EditEntries(Resource):
    decorators = [jwt_required(), limiter.limit("40/second")]
    def get(self):
        uploaded_entry_ids = current_identity.id["uploaded"]

        global entries
        own_entries = {}
        for entry_id in uploaded_entry_ids:
            own_entries[entry_id] = entries[entry_id]

        page = 0
        if request.args.get("page"):
            try:
                page = int(request.args.get("page"))
            except:
                pass

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

class EntryCount(Resource):
    #@jwt_required()
    decorators = [jwt_required(), limiter.limit("40/second")]
    def get(self):
        return {"entry_count": len(entries)}

class Entry(Resource):
    #@jwt_required()
    decorators = [jwt_required(), limiter.limit("10/second")]
    def get(self, uid: int):
        global entries

        # reversing the id with len(entries) - 1 - correct id,
        # because we also reverse the entries list beforehand

        # using an index won't work if an entry is deleted
        # -> if all releated idxs would be updated, many links to an upload may become invalid
        #entry = entries[len(entries) - 1 - uid]
        #if entry["view"]:
        #    return 404

        with open(f"{PATH}/static/uploaded/{uid}.json", mode='r') as file:
            entry = json.load(file)
             
            if entry["view"]:
                return 404

            return entry
        
