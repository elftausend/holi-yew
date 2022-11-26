from typing import Dict, List
#from flask_sqlalchemy import SQLAlchemy
import sqlite3
import json
from flask import jsonify
from flask_restful import request, Resource
from flask_jwt_extended import jwt_required, current_user

from holiapi.config import config, PATH
from holiapi.api_limiter import limiter



USER_DB = f"{PATH}/db/user_database.db"

class UserRoute(Resource):
    decorators = [jwt_required(), limiter.limit("20/second")]
    def get(self):
        token = request.headers["Authorization"][7:]

        # return entire current_user.as_dict()?
        dict= {
            "user_id": current_user.username,
            "division": current_user.htl_division,
            "token": token,
            "uploaded": current_user.uploaded,
            "favs": current_user.favs,
            "htl_class": current_user.htl_class,
            "upload_banned": current_user.upload_banned,
            "is_admin": current_user.is_admin()
        }
        print(f"user info: {dict}")
        return dict


#db = SQLAlchemy()

class UserInfo():
    def __init__(self, access_token: str, username: str, user_id: str, htl_class: str, htl_division: str, htl_type: str, uploaded=[], favs = []):
        self.id = access_token
        self.username = username
        self.user_id = user_id
        self.htl_class = htl_class
        self.htl_division = htl_division
        self.htl_type = htl_type
        self.uploaded = uploaded
        self.favs = favs

    def set_uploaded_and_favs(self, db_results: Dict[str, List[int]]):
        self.uploaded = db_results["uploaded"]
        self.favs = db_results["fav"]

#db.Model
class User():
    #user_id = db.Column(db.Integer, primary_key=True)
    # list
    # stars = db.Column()

    def __init__(self, htl_access_token: str, username: str, user_id: str, htl_class: str, htl_division: str,  upload_banned: bool, uploaded=[], favs = []):
        self.htl_access_token = htl_access_token
        self.username = username
        self.user_id = user_id
        self.htl_class = htl_class
        self.htl_division = htl_division
        self.uploaded = uploaded
        self.favs = favs
        self.is_config_admin = self.user_id in config.admin_ids
        self.upload_banned = upload_banned

    def set_uploaded_and_favs(self, db_results: Dict[str, List[int]]):
        self.uploaded = db_results["uploaded"]
        self.favs = db_results["fav"]

    def is_admin(self) -> bool:
        return self.is_config_admin
    
    def is_banned(self) -> bool:
        return self.user_id in config.banned_ids
    
    def is_whitelisted(self) -> bool:
        return self.user_id in config.whitelist_ids

    def as_dict(self):
        return {
            "htl_access_token": self.htl_access_token,
            "username": self.username,
            "user_id": self.user_id,
            "htl_class": self.htl_class,
            "htl_division": self.htl_division,
            "uploaded": self.uploaded,
            "favs": self.favs
        }


def get_user_from_raw(user_info_raw, access_token: str) -> User:
    # TODO: remember
    
    # personal name
    username = user_info_raw["0"]["displayname"]["0"]
    htl_related_ids = user_info_raw["0"]["dn"].split(",")
    
    # id (e.g. 101234)
    user_id = str(htl_related_ids[0][3:])
    # 2AHMBT
    htl_class = htl_related_ids[1][3:]

    # abteilung (ME)
    htl_division = htl_related_ids[2][3:]

    # to differentiate between Wirtschaftsing. logistik and informatik
    # WI -> WIL || WII
    if htl_division == "WI":
        htl_division = htl_class[3:]

    # no access for lebenmittel
    if htl_division == "L":
        return None

    # root, useless (HTBL) 
    htl_type = htl_related_ids[3][2:]

    return User(access_token, username, user_id, htl_class, htl_division, htl_type)


def query_db_results(user: User, db = USER_DB) -> Dict[str, List[int]]:
    # use ORM
    con = sqlite3.connect(db)
    cur = con.cursor()

    cur.execute("select * from users where user_id=?", (user.user_id,))
    data = cur.fetchall()
    
    db_results = '{"uploaded": [], "fav": []}'
    if data:
        db_results = data[0][2]
    else:
        cur.execute("insert into users (user_id, username, entry_info, class) values(?, ?, ?, ?)",
        (user.user_id, user.username, json.dumps({ "uploaded": [], "fav": [] }), user.htl_class))
        con.commit()
    
    print(f"db_results: {db_results}")
    con.close()
    return json.loads(db_results)
