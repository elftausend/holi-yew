import os

PATH = os.path.dirname(os.path.realpath(__file__))

def create_user_db(path = PATH):
    if not os.path.exists(f"{path}/user_database.db"):
        os.system(f"sqlite3 {path}/user_database.db < {PATH}/user_schema.sql")