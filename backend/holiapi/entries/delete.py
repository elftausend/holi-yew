from holiapi.api_limiter import limiter
from holiapi.utils import entries
from holiapi.config import PATH
from holiapi.user import User
from holiapi.db.entry_info import write_entry_info_to_db
from holiapi.logger import log

from os import remove
from flask import request
from flask_jwt_extended import current_user, jwt_required
from flask_restful import Resource

def delete_entry(uid: int):
    entry_info = entries[uid]
    entries.entries.pop(uid)

    # remove entry info json file
    remove(f"{PATH}/static/uploaded/{uid}.json")

    try:
        # remove uploaded file (pdfs, source files)
        remove(f"{PATH}/static/files/{entry_info['hash']}.{entry_info['ext']}")
    except:
        pass

    try:
        # remove extracted images of the pdf
        for idx, img_ext in enumerate(entry_info["img_exts"]):
            remove(f"{PATH}/static/images/{entry_info['hash']}_{idx}.{img_ext}")
    except:
        pass

def remove_entry_from_user_uploaded(uid: int, user: User):
    # an admin may delete an entry, only then can this uid be removed
    if uid in user.uploaded:
        user.uploaded.remove(uid)

    print(f"uploaded: {user.uploaded}")

    entry_info = {
        "uploaded": user.uploaded,
        "fav": user.favs
    }
    write_entry_info_to_db(user.user_id, entry_info)


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
        
        entry_info = entries[uid]
        title = entry_info["title"]
        tags = entry_info["tags"]
        hash = entry_info["hash"]
        
        log(f"{current_user.user_id}/{current_user.username}/{current_user.htl_class} removed entry called '{title}' with tags '{tags}' and hash '{hash}/{uid}'.")
        remove_entry_from_user_uploaded(uid, current_user)
        delete_entry(uid)
        