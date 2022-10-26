from typing import Dict, List
import requests
from utils import file_contents
from flask_sqlalchemy import SQLAlchemy
from config import config
import sqlite3
import os
import json

TOKEN_URL = "https://auth.htl-hl.ac.at/token.php"
CLIENT_ID = "holi.htl-hl.ac.at"
CLIENT_SECRET = file_contents("client_secret")
GRANT_TYPE = "authorization_code"
REDIRECT_URI = "https://holi.htl-hl.ac.at/authenticated"

USER_INFO_URL = "https://auth.htl-hl.ac.at/getUserInformation.php?access_token="

PATH = os.path.dirname(os.path.realpath(__file__))
USER_DB = f"{PATH}/db/user_database.db"

db = SQLAlchemy()

def query_db_results(user_id: str) -> Dict[str, List[int]]:
    # use ORM
    con = sqlite3.connect(USER_DB)
    cur = con.cursor()

    cur.execute("select * from users where user_id=?", (user_id,))
    data = cur.fetchall()
    
    db_results = '{"uploaded": [], "fav": []}'
    if data:
        db_results = data[0][1]
    else:
        cur.execute("insert into users (user_id, entry_info) values(?, ?)", (user_id, json.dumps({ "uploaded": [], "fav": [] })))
        con.commit()
    
    con.close()
    return json.loads(db_results)

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

def get_user_info(access_token: str) -> UserInfo:
    # TODO: remember
    #user_info = requests.get(f"{USER_INFO_URL}{access_token}").json()

    user_info = {'count': 1, '0': {'mail': {'count': 2, '0': 'email1', '1': 'email2'}, '0': 'mail', 'displayname': {'count': 1, '0': 'A Name'}, '1': 'displayname', 'count': 2, 'dn': 'cn=111111,ou=1AFET,ou=ET,o=HTBL'}}

    # personal name
    username = user_info["0"]["displayname"]["0"]
    htl_related_ids = user_info["0"]["dn"].split(",")
    
    # id (e.g. 101234)
    user_id = str(htl_related_ids[0][3:])
    # 2AHMBT
    htl_class = htl_related_ids[1][3:]
    # abteilung (ME)
    htl_division = htl_related_ids[2][3:]
    # root (HTBL)
    htl_type = htl_related_ids[3][2:]

    return UserInfo(access_token, username, user_id, htl_class, htl_division, htl_type)

#db.Model
class User():
    #user_id = db.Column(db.Integer, primary_key=True)
    # list
    # stars = db.Column()

    def __init__(self, user_info):
        self.id = {
            "username": user_info.username,
            "token": user_info.id,
            "user_id": user_info.user_id,
            "htl_class": user_info.htl_class,
            "htl_division": user_info.htl_division,
            "htl_type": user_info.htl_type,
            "uploaded": user_info.uploaded,
            "favs": user_info.favs
        }

def authenticate(username, code):
    # auth with htlhl
    print(f"received code: {code}")
#
    #payload = {
    #    "client_id": CLIENT_ID,
    #    "client_secret": CLIENT_SECRET,
    #    "grant_type": GRANT_TYPE,
    #    "code": code,
    #    "redirect_uri": REDIRECT_URI,
    #}
#
    #answer = requests.post(TOKEN_URL, json=payload)
    #if not answer:
    #    return
#
    #token_info = answer.json()

    # TODO: remember
    #user_info = get_user_info(token_info["access_token"])
    user_info = get_user_info("remember")

    # if user is banned, don't authenticate
    if user_info.user_id in config.banned_ids:
        return

    user_info.set_uploaded_and_favs(query_db_results(user_info.user_id))
    return User(user_info)

def identity(payload):
    user_info_dict = payload['identity']
    return User(
        UserInfo(
            access_token=user_info_dict["token"],
            username=user_info_dict["username"],
            user_id=user_info_dict["user_id"],
            htl_class=user_info_dict["htl_class"],
            htl_division=user_info_dict["htl_division"],
            htl_type=user_info_dict["htl_type"],
            uploaded=user_info_dict["uploaded"],
            favs=user_info_dict["favs"]
        )
    )
