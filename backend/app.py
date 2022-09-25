import os
from flask import Flask, request, jsonify
from flask_restful import Resource, Api
from flask_cors import CORS
from flask_jwt import JWT, jwt_required, current_identity
#import jwt

app = Flask(__name__)
CORS(app)
api = Api(app)
app.secret_key = os.urandom(32)

class User(object):
    def __init__(self, id):
        self.id = id

def authenticate(username, password):
    # auth with htlhl
    user = User()
    user.id = username
    return user

def identity(payload):
    user_id = payload['identity']
    user = User()
    user.id = user_id
    return user

jwt = JWT(app, authenticate, identity)

class User(Resource):
    @jwt_required()
    def get(self):
        token = request.headers["Authorization"][4:]
        x = {"user_id" : current_identity.id, "token" : token}
        print(f"user get: {x}")
        return jsonify({"user_id" : current_identity.id, "token" : token})

class Entry(Resource):
    @jwt_required()
    def get(self):
        a = request.headers["Authorization"][4:]
        print(f"{a}")
        
api.add_resource(User, '/user')
api.add_resource(Entry, '/entry')

if __name__ == '__main__':
    app.run(debug=True, port=8011)