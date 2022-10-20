import requests
from utils import file_contents

TOKEN_URL = "https://auth.htl-hl.ac.at/token.php"
CLIENT_ID = "holi.htl-hl.ac.at"
CLIENT_SECRET = file_contents("client_secret")
GRANT_TYPE = "authorization_code"
REDIRECT_URI = "https://holi.htl-hl.ac.at/authenticated"

USER_INFO_URL = "https://auth.htl-hl.ac.at/getUserInformation.php?access_token="

class User(object):
    def __init__(self, id):
        self.id = id

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

    username = 200151 # from token
    return User(username)

def identity(payload):
    user_id = payload['identity']
    return User(user_id)

code = "8616825d1efc87872792f9930300137f085b5d57"

print(f"client_secret: {CLIENT_SECRET}")

payload = {
    "client_id": CLIENT_ID,
    "client_secret": CLIENT_SECRET,
    "grant_type": GRANT_TYPE,
    "code": code,
    "redirect_uri": REDIRECT_URI,
}

answer = requests.post(TOKEN_URL, json=payload)
if answer:

    json = answer.json()
    access_token = json["access_token"]
    print(f"answer: {answer} {answer.status_code}")

    user_info = requests.get(f"{USER_INFO_URL}{access_token}")
    print(f"user info: {user_info.json()}")