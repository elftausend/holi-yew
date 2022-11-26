from flask_restful import Resource, request
from flask_jwt_extended import current_user, jwt_required

from holiapi.db.db_fns import add_uid_to_favs
from holiapi.api_limiter import limiter
from holiapi.upload import save_upload_dict_as_json
from holiapi import utils

class Favo(Resource):
    decorators = [jwt_required(), limiter.limit("45/second")]
    def post(self):
        uid = request.args.get("uid")
        if not uid:
            return

        try:
            uid = int(uid)
        except:
            return 400

        if uid in current_user.favs:
            return

        entry_info = utils.entries[uid]
        entry_info["favs"] += 1
        save_upload_dict_as_json(entry_info, uid)

        add_uid_to_favs(uid, current_user)
    