import sqlite3
import json
from holiapi.config import PATH
from holiapi.user import User

USER_DB = f"{PATH}/db/user_database.db"

def write_entry_info_to_db(user_id: str, entry_info, db = USER_DB):
    con = sqlite3.connect(db)
    cur = con.cursor()

    cur.execute("update users set entry_info = ? where user_id=?", (json.dumps(entry_info), user_id))
    con.commit()
    con.close()