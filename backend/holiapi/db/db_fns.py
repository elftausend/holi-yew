import sqlite3

from holiapi.config import PATH

USER_DB = f"{PATH}/db/user_database.db"

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
        
incr_flag_count_by_x("111111", 2)
incr_flag_count_by_x("111111", 1)