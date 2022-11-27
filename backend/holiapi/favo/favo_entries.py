from flask_restful import Resource, request
from flask_jwt_extended import current_user, jwt_required

from holiapi.entries import filter_tags
from holiapi.api_limiter import limiter
from holiapi import utils

class Favo(Resource):
    decorators = [jwt_required(), limiter.limit("40/second")]
    def get(self):
        local_entries = {}

        for fav in current_user.favs:
            local_entries[fav] = utils.entries[fav]
        
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
            local_entries = filter_tags.filter_for_tags(returned_tags, local_entries)
        else:
            tags = ""

        if page*16 >= len(local_entries):
            return {}
        start, end, page_count = utils.limit_end_len(page, len(local_entries))
        if page > page_count or page < 0:
            return 400
        
        return {
            "entries": list(local_entries.values())[start:end],
            "page_count": page_count
        }
        