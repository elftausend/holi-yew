import unittest
import os

from holiapi.db.setup_db import create_user_db
from holiapi.upload import add_upload_id_to_db
from holiapi.user import query_db_results, get_user_from_raw, User
from holiapi.db.entry_info import write_entry_info_to_db

DB = "./db/user_database.db"

class TestDatabase(unittest.TestCase):
    def test_query_db(self):
        delete_db()
        create_user_db("db/")
        
        uids = query_db_results("11111", "A Name", DB)
        assert uids["uploaded"] == []
        assert uids["fav"] == []

        # checks if entry in db was created
        uids = query_db_results("11111", "A Name", DB)
        assert uids["uploaded"] == []
        assert uids["fav"] == []

        os.remove(DB)

    def test_write_entry_info(self):
        delete_db()
        create_user_db("db/")

        uids = query_db_results("11111", "A Name", DB)
        assert uids["uploaded"] == []
        assert uids["fav"] == []

        uids["uploaded"].append(2)

        write_entry_info_to_db("11111", uids, db=DB)

        uids = query_db_results("11111", "A Name", DB)
        assert uids["uploaded"] == [2]
        assert uids["fav"] == []

        os.remove(DB)

    def test_add_upload_id_to_db(self):
        delete_db()
        create_user_db("db/")

        user_info_raw = {
            'count': 1, 
            '0': {'mail': {'count': 2, '0': 'email1', '1': 'email2'}, 
            '0': 'mail', 'displayname': {'count': 1, '0': 'A Name'}, 
            '1': 'displayname', 'count': 2, 'dn': 'cn=111211,ou=1AHWII,ou=WI,o=HTBL'}
        }

        user = get_user_from_raw(user_info_raw, "notoken")

        
        uids = query_db_results(user.user_id, user.username, DB)
        
        assert uids["uploaded"] == []
        assert uids["fav"] == []

        user.set_uploaded_and_favs(uids)
        

#        add_upload_id_to_db(5, user)

 #       uids = query_db_results(user_info.user_id, DB)
  #      assert uids["uploaded"] == [5]
  #      assert uids["fav"] == []

        delete_db()


# if a test failed beforehand, delete the database
def delete_db():
    try:
        os.remove(DB)
    except:
        pass
