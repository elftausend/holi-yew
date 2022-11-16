from holiapi.db.db_fns import get_users

from flask_restful import Resource
from flask_jwt_extended import current_user, jwt_required
from holiapi.api_limiter import limiter

class Users(Resource):
    decorators = [jwt_required(), limiter.limit("50/second")]
    def get(self):
        if not current_user.is_admin():
            return 403
        
        users = get_users()
        users_json = []
        for user in users:
            users_json.append(
                {
                    "usid": user[0],
                    "username": user[1],
                    "flag_count": user[3],
                }
            )

        return users_json