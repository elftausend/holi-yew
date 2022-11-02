from holiapi.api_limiter import limiter
from holiapi.utils import entries
from holiapi.config import PATH

from os import remove
from flask import request
from flask_jwt_extended import current_user, jwt_required
from flask_restful import Resource


def delete_entry(uid: int):
    entry_info = entries[uid]

    # remove uploaded file (pdfs, source files)
    remove(f"{PATH}/static/files/{entry_info['hash']}.{entry_info['ext']}")
    # remove entry info json file
    remove(f"{PATH}/static/uploaded/{uid}.json")

    # remove extracted images of the pdf
    for img_ext in entry_info["img_exts"]:
        remove(f"{PATH}/static/images/{entry_info['hash']}{img_ext}")

    entries.pop(uid)

class DeleteEntry(Resource):
    decorators = [jwt_required(), limiter.limit("32/minute")]
    def post(self):
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
        
        delete_entry(uid)
        

    
    