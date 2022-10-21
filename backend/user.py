import requests
from utils import file_contents
from flask_sqlalchemy import SQLAlchemy

TOKEN_URL = "https://auth.htl-hl.ac.at/token.php"
CLIENT_ID = "holi.htl-hl.ac.at"
CLIENT_SECRET = file_contents("client_secret")
GRANT_TYPE = "authorization_code"
REDIRECT_URI = "https://holi.htl-hl.ac.at/authenticated"

USER_INFO_URL = "https://auth.htl-hl.ac.at/getUserInformation.php?access_token="

db = SQLAlchemy()


class UserInfo():
    def __init__(self, access_token: str, username: str, user_id: int, htl_class: str, htl_division: str, htl_type: str):
        self.id = access_token
        self.username = username
        self.user_id = user_id
        self.htl_class = htl_class
        self.htl_division = htl_division
        self.htl_type = htl_type

def get_user_info(access_token: str) -> UserInfo:
    user_info = requests.get(f"{USER_INFO_URL}{access_token}").json()
    # personal name
    username = user_info["0"]["displayname"]["0"]
    htl_related_ids = user_info["0"]["dn"].split(",")
    
    # id (e.g. 101234)
    user_id = htl_related_ids[0][3:]
    # 2AHMBT
    htl_class = htl_related_ids[1][3:]
    # abteilung (ME)
    htl_division = htl_related_ids[2][3:]
    # lebensmittel oder bundes htl (HTBL) denke ich
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
            "htl_type": user_info.htl_type
        }

def authenticate(username, code):
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

    token_info = answer.json()

    user_info = get_user_info(token_info["access_token"])
    return User(user_info)

def identity(payload):
    print(f"payload: {payload}")
    user_info_dict = payload['identity']
    return User(
        UserInfo(
            access_token=user_info_dict["token"],
            username=user_info_dict["username"],
            user_id=user_info_dict["user_id"],
            htl_class=user_info_dict["htl_class"],
            htl_division=user_info_dict["htl_division"],
            htl_type=user_info_dict["htl_type"]
        )
    )
