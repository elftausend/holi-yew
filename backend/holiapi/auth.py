import requests

from flask import jsonify
from flask_restful import Resource, request
from flask_jwt_extended import JWTManager, create_access_token
from holiapi.utils import file_contents
from holiapi.user import query_db_results, User, get_user_from_raw
from holiapi.config import config

TOKEN_URL = "https://auth.htl-hl.ac.at/token.php"
CLIENT_ID = "holi.htl-hl.ac.at"
CLIENT_SECRET = file_contents("client_secret")
GRANT_TYPE = "authorization_code"
REDIRECT_URI = "https://holi.htl-hl.ac.at/authenticated"

USER_INFO_URL = "https://auth.htl-hl.ac.at/getUserInformation.php?access_token="

jwt = JWTManager()

@jwt.user_identity_loader
def user_identity_lookup(user):
    print(f"user {user}")
    return user

@jwt.user_lookup_loader
def user_lookup_callback(_jwt_header, jwt_data):
    user_info_dict = jwt_data["sub"]
    print(f"identity: {user_info_dict}")
    
    uploaded_and_favs = query_db_results(user_info_dict["user_id"])    

    return User(
        htl_access_token=user_info_dict["htl_access_token"],
        username=user_info_dict["username"],
        user_id=user_info_dict["user_id"],
        htl_class=user_info_dict["htl_class"],
        htl_division=user_info_dict["htl_division"],
        uploaded=uploaded_and_favs["uploaded"],
        favs=uploaded_and_favs["fav"]
    )

class Auth(Resource):
    def post(self):
        code = request.json.get("code", None)
        
        # auth with htlhl
        print(f"received code: {code}")

        payload = {
            "client_id": CLIENT_ID,
            "client_secret": CLIENT_SECRET,
            "grant_type": GRANT_TYPE,
            "code": code,
            "redirect_uri": REDIRECT_URI,
        }
        answer = requests.post(TOKEN_URL, json=payload)
        if not answer:
            return

        token = answer.json()["access_token"]

        user_info_raw = requests.get(f"{USER_INFO_URL}{token}").json()

        #user_info_raw = {'count': 1, '0': {'mail': {'count': 2, '0': 'email1', '1': 'email2'}, '0': 'mail', 'displayname': {'count': 1, '0': 'A Name'}, '1': 'displayname', 'count': 2, 'dn': 'cn=111111,ou=1AFET,ou=ET,o=HTBL'}}
        #token = "asdfas"

        user = get_user_from_raw(user_info_raw, token)

        # attaining user_info was not successful
        if not user:
            return

        if config.whitelist and not user.is_whitelisted():
            return

        # if user is banned, don't authenticate
        if user.is_banned():
            return


        user.set_uploaded_and_favs(query_db_results(user.user_id))
        access_token = create_access_token(identity=user.as_dict())
        return jsonify(access_token=access_token)
