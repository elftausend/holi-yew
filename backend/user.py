class User(object):
    def __init__(self, id):
        self.id = id

def authenticate(username, token):
    # auth with htlhl
    print(f"received token: {token}")
    username = 200151 # from token
    return User(username)

def identity(payload):
    user_id = payload['identity']
    return User(user_id)