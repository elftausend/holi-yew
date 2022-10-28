import os
import datetime
from flask import Flask, request, jsonify
from flask_restful import Resource, Api, request
from flask_cors import CORS, cross_origin
from flask_jwt import JWT, jwt_required, current_identity
#import jwt

from holiapi.entries.entries import Entries, Entry, EntryCount
from holiapi.entries.edit import EditEntries, EditEntry
from holiapi.upload import *
from holiapi.user import *

from holiapi.api_limiter import limiter

class UserRoute(Resource):
    #@jwt_required()
    decorators = [jwt_required(), limiter.limit("20/second")]
    def get(self):
        token = request.headers["Authorization"][4:]

        #user_info = requests.get(f"{USER_INFO_URL}{current_identity.id}").json()
        #username = user_info["0"]["displayname"]["0"]
        #return user_info
        return jsonify({
            "user_id": current_identity.id["username"],
            "division": current_identity.id["htl_division"],
            "token": token,
            "uploaded": current_identity.id["uploaded"],
            "favs": current_identity.id["favs"]
        })


def init_and_run():
    app = Flask(__name__)

    # User database init
    # db.init_app(app)

    # Limiter init
    limiter.init_app(app)

    api = Api(app)
    app.secret_key = os.urandom(32)

    # can be disabled for HTL HL (due to trunk proxy config)
    CORS(app)
    #app.config['CORS_HEADERS'] = 'Content-Type'

    #app.config["SQLALCHEMY_DATABASE_URI"] = "sqlite:///user.db"
    app.config["JWT_EXPIRATION_DELTA"] = datetime.timedelta(minutes=24*60*7)
    app.config["JWT_AUTH_PASSWORD_KEY"] = "code"
    #app.config["JWT_AUTH_URL_RULE"] = "/api/auth"


    jwt = JWT(app, authenticate, identity)

    api.add_resource(UserRoute, '/user')
    api.add_resource(Entries, '/entries')
    api.add_resource(EntryCount, '/entry_count')
    api.add_resource(Entry, '/entry/<int:uid>')
    api.add_resource(Upload, '/upload')
    api.add_resource(EditEntries, "/editable_entries")
    api.add_resource(EditEntry, "/edit_entry")

    from holiapi.db.setup_db import create_user_db
    create_user_db()
    #from waitress import serve
    #serve(app, host="127.0.0.1", port=82, threads=16)
    app.run(debug=True, port=82)




#if __name__ == '__main__':
    