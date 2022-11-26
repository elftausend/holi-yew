import sqlite3

from holiapi.config import PATH
from holiapi.user import User
from holiapi.db.entry_info import write_entry_info_to_db

USER_DB = f"{PATH}/db/user_database.db"

def get_users(db = USER_DB):
    con = sqlite3.connect(db)
    cur = con.cursor()

    cur.execute("select * from users;")
    users = cur.fetchall()

    con.commit()
    con.close()

    return users

def incr_flag_count_by_x(user_id: str, x: int, db = USER_DB):
    con = sqlite3.connect(db)
    cur = con.cursor()

    cur.execute("update users set flag_count = flag_count + ? where user_id=?", (x, user_id))

    con.commit()
    con.close()

def set_flag_count(user_id: str, flag_count: int, db = USER_DB):
    con = sqlite3.connect(db)
    cur = con.cursor()

    cur.execute("update users set flag_count = ? where user_id=?", (flag_count, user_id))

    con.commit()
    con.close()


def get_flag_count(user_id: str, db = USER_DB) -> int:
    con = sqlite3.connect(db)
    cur = con.cursor()

    cur.execute("select flag_count from users where user_id=?", (user_id,))
    flag_count = cur.fetchall()[0][0]

    con.commit()
    con.close()

    return flag_count

def set_banned(user_id: str, banned: int, db = USER_DB):
    con = sqlite3.connect(db)
    cur = con.cursor()

    cur.execute("update users set banned = ? where user_id=?", (banned, user_id))

    con.commit()
    con.close()

def get_upload_banned(user_id: str, db = USER_DB) -> bool:
    flag_count = get_flag_count(user_id, db)
    return flag_count >= 3

def add_uid_to_favs(uid: int, user: User):
    if uid in user.favs:
        return

    user.favs.append(uid)

    entry_info = {
        "uploaded": user.uploaded,
        "fav": user.favs
    }
    print(f"entry_info: {entry_info}")

    write_entry_info_to_db(user.user_id, entry_info)

def remove_uid_from_favs(uid: int, user: User):
    if uid not in user.favs:
        return    

    user.favs.remove(uid)

    entry_info = {
        "uploaded": user.uploaded,
        "fav": user.favs
    }

    write_entry_info_to_db(user.user_id, entry_info)

#incr_flag_count_by_x("111111", 2)
#incr_flag_count_by_x("111111", 1)