import os
import datetime
from flask import Flask, request, jsonify
from flask_restful import Resource, Api, request
from flask_cors import CORS, cross_origin
from flask_jwt import JWT, jwt_required, current_identity
#import jwt

from entries import *
from upload import *
from user import *

from api_limiter import limiter
import config

app = Flask(__name__)

# User database init
# db.init_app(app)

# Limiter init
limiter.init_app(app)

PATH = os.path.dirname(os.path.realpath(__file__))
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
            "token": token
        })

        

api.add_resource(UserRoute, '/user')
api.add_resource(Entries, '/entries')
api.add_resource(EntryCount, '/entry_count')
api.add_resource(Entry, '/entry/<string:hash>')
api.add_resource(Upload, '/upload')

if __name__ == '__main__':
    #from waitress import serve
    #serve(app, host="127.0.0.1", port=82, threads=16)
    app.run(debug=True, port=82)