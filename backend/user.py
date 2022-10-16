class User(object):
    def __init__(self, id):
        self.id = id

def authenticate(username, password):
    # auth with htlhl
    return User(username)

def identity(payload):
    user_id = payload['identity']
    return User(user_id)