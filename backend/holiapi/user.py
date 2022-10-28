from typing import Dict, List
from holiapi.utils import file_contents
from flask_sqlalchemy import SQLAlchemy
from holiapi.config import config, PATH
import sqlite3
import json

TOKEN_URL = "https://auth.htl-hl.ac.at/token.php"
CLIENT_ID = "holi.htl-hl.ac.at"
CLIENT_SECRET = file_contents("client_secret")
GRANT_TYPE = "authorization_code"
REDIRECT_URI = "https://holi.htl-hl.ac.at/authenticated"

USER_INFO_URL = "https://auth.htl-hl.ac.at/getUserInformation.php?access_token="

USER_DB = f"{PATH}/db/user_database.db"

db = SQLAlchemy()

def query_db_results(user_id: str, db = USER_DB) -> Dict[str, List[int]]:
    # use ORM
    con = sqlite3.connect(db)
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

def get_user_info(user_info_raw, access_token: str) -> UserInfo:
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

    def is_admin(self):
        return self.id["user_id"] in config.admin_ids
    
    def is_banned(self):
        return self.id["user_id"] in config.banned_ids

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
    #token = answer.json()["access_token"]
#
    #user_info_raw = requests.get(f"{USER_INFO_URL}{token}").json()

    # TODO: remember
    user_info_raw = {'count': 1, '0': {'mail': {'count': 2, '0': 'email1', '1': 'email2'}, '0': 'mail', 'displayname': {'count': 1, '0': 'A Name'}, '1': 'displayname', 'count': 2, 'dn': 'cn=111111,ou=1AFET,ou=ET,o=HTBL'}}
    token = "asdfas"

    user_info = get_user_info(user_info_raw, token)

    # attaining user_info was not successful
    if not user_info:
        return

    #user_info = get_user_info("remember")

    # if user is banned, doyn't authenticate
    if user_info.user_id in config.banned_ids:
        return

    user_info.set_uploaded_and_favs(query_db_results(user_info.user_id))
    return User(user_info)

def identity(payload):
    user_info_dict = payload['identity']

    # appending uploaded entry uids to current_identity does not add it "globally"
    # modify payload?
    uploaded_and_favs = query_db_results(user_info_dict["user_id"])
    return User(
        UserInfo(
            access_token=user_info_dict["token"],
            username=user_info_dict["username"],
            user_id=user_info_dict["user_id"],
            htl_class=user_info_dict["htl_class"],
            htl_division=user_info_dict["htl_division"],
            htl_type=user_info_dict["htl_type"],
            uploaded=uploaded_and_favs["uploaded"],
            favs=uploaded_and_favs["fav"]
        )
    )
