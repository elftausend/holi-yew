import os
import datetime
from flask import Flask, request, jsonify
from flask_restful import Resource, Api, request
from flask_cors import CORS, cross_origin
from flask_jwt import JWT, jwt_required, current_identity
#import jwt
import utils
import json
import filter_tags
from upload import *
from user import *
from utils import entries

app = Flask(__name__)
PATH = os.path.dirname(os.path.realpath(__file__))
api = Api(app)
app.secret_key = os.urandom(32)

# can be disabled for HTL HL (due to trunk proxy config)
CORS(app)
#app.config['CORS_HEADERS'] = 'Content-Type'

app.config["JWT_EXPIRATION_DELTA"] = datetime.timedelta(minutes=24*60*7)
app.config["JWT_AUTH_PASSWORD_KEY"] = "code"
#app.config["JWT_AUTH_URL_RULE"] = "/api/auth"


jwt = JWT(app, authenticate, identity)

class UserRoute(Resource):
    @jwt_required()
    def get(self):
        token = request.headers["Authorization"][4:]
        x = {"user_id" : current_identity.id, "token" : token}
        #print(f"user get: {x}")
        return jsonify({"user_id" : str(current_identity.id), "token" : token})

class Entries(Resource):
    @jwt_required()
    def get(self):

        global entries
        local_entries = entries

        page = 0
        if request.args.get("page"):
            try:
                page = int(request.args.get("page"))
            except:
                pass

        tags = request.args.get("tags")
        if tags:
            returned_tags = tags.split()
            print(returned_tags)
            local_entries = filter_tags.filter_for_tags(returned_tags, entries)
        else:
            tags = ""

        if page*16 >= len(entries):
            return {}
        start, end, page_count = utils.limit_end_len(page, len(entries))
        if page > page_count or page < 0:
            return 400
        
        # return page count as well
        return local_entries[start:end]

class EntryCount(Resource):
    @jwt_required()
    def get(self):
        return {"entry_count": len(entries)}

class Entry(Resource):
    @jwt_required()
    def get(self, hash: str):
        upload = hash + ".json"
    
        # check if exists?
        with open(f"{PATH}/static/uploaded/{upload}", mode='r') as file:
            return json.load(file)
        

api.add_resource(UserRoute, '/user')
api.add_resource(Entries, '/entries')
api.add_resource(EntryCount, '/entry_count')
api.add_resource(Entry, '/entry/<string:hash>')
api.add_resource(Upload, '/upload')

if __name__ == '__main__':
    #from waitress import serve
    #serve(app, host="127.0.0.1", port=82, threads=16)
    app.run(debug=True, port=82)