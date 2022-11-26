from flask_restful import Resource, request
from flask_jwt_extended import current_user, jwt_required

from holiapi.db.db_fns import add_uid_to_favs
from holiapi.api_limiter import limiter

class Favo(Resource):
    decorators = [jwt_required(), limiter.limit("26/second")]
    def post(self):
        uid = request.args.get("uid")
        if not uid:
            return

        try:
            uid = int(uid)
        except:
            return 400
        
        add_uid_to_favs(uid, current_user)
    