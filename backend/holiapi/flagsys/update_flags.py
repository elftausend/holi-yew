from holiapi.db.db_fns import incr_flag_count_by_x, get_flag_count, set_flag_count
from holiapi.api_limiter import limiter
from flask_jwt_extended import jwt_required, current_user
from flask_restful import Resource, request

class FlagUpdate(Resource):
    decorators = [jwt_required(), limiter.limit("30/second")]
    def post(self):
        if not current_user.is_admin():
            return 403
        
        json_data = request.get_json(force=True)
        
        flag_incr = json_data["flag_incr"]
        usid = json_data["usid"]

        incr_flag_count_by_x(usid, flag_incr)

        if get_flag_count(usid) < 0:
            set_flag_count(usid, 0)

        return {"flag_count": get_flag_count(usid)}
