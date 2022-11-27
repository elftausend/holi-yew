#import holiapi.entries.filter_tags
from holiapi import utils
from holiapi.entries import filter_tags
from flask import jsonify
from flask_restful import Resource, request
from flask_jwt_extended import jwt_required
from holiapi.utils import entries
from holiapi.config import PATH

from holiapi.api_limiter import limiter

class Entries(Resource):
    #@jwt_required()
    decorators = [jwt_required(), limiter.limit("40/second")]
    def get(self):

        local_entries = entries

        page = 0
        if request.args.get("page"):
            try:
                page = int(request.args.get("page"))
            except:
                return 400

        tags = request.args.get("tags")
        if tags:
            returned_tags = tags.split()
            print(returned_tags)
            local_entries = filter_tags.filter_for_tags(returned_tags, entries)
        else:
            tags = ""

        if page*16 >= len(entries):
            return {}
        start, end, page_count = utils.limit_end_len(page, len(local_entries))
        if page > page_count or page < 0:
            return 400
        
        # return page count as well
        #return list(local_entries.values())[start:end]
        return {
            "entries": list(local_entries.values())[start:end],
            "page_count": page_count
        }


class EntryCount(Resource):
    #@jwt_required()
    decorators = [jwt_required(), limiter.limit("40/second")]
    def get(self):
        return jsonify({"entry_count": len(entries)})

class Entry(Resource):
    #@jwt_required()
    decorators = [jwt_required(), limiter.limit("10/second")]
    def get(self, uid: int):

        # reversing the id with len(entries) - 1 - correct id,
        # because we also reverse the entries list beforehand

        # using an index won't work if an entry is deleted
        # -> if all releated idxs would be updated, many links to an upload may become invalid
        #entry = entries[len(entries) - 1 - uid]
        #if entry["view"]:
        #    return 404

        try:
            return entries[uid]
        except Exception:
            return 404
        
