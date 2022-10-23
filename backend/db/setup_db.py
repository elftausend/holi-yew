import os

PATH = os.path.dirname(os.path.realpath(__file__))

def create_user_db():
    if not os.path.exists(f"{PATH}/user_database.db"):
        os.system(f"sqlite3 {PATH}/user_database.db < {PATH}/user_schema.sql")