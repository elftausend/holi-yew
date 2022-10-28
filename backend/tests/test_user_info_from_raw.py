import unittest
import pytest

from holiapi.user import get_user_info

class TestGetUserInfo(unittest.TestCase):
    def test_get_user_info_from_raw(self):
        user_info_raw = {
            'count': 1, 
            '0': {'mail': {'count': 2, '0': 'email1', '1': 'email2'}, 
            '0': 'mail', 'displayname': {'count': 1, '0': 'A Name'}, 
            '1': 'displayname', 'count': 2, 'dn': 'cn=111111,ou=1AFET,ou=ET,o=HTBL'}
        }
        
        user_info = get_user_info(user_info_raw, "notoken")

        assert user_info.htl_class == "1AFET"
        assert user_info.htl_division == "ET"
        assert user_info.username == "A Name"
        assert user_info.id == "notoken"
        assert user_info.user_id == "111111"

    def test_get_ui_wii(self):
        user_info_raw = {
            'count': 1, 
            '0': {'mail': {'count': 2, '0': 'email1', '1': 'email2'}, 
            '0': 'mail', 'displayname': {'count': 1, '0': 'A Name'}, 
            '1': 'displayname', 'count': 2, 'dn': 'cn=111111,ou=1AHWII,ou=WI,o=HTBL'}
        }

        user_info = get_user_info(user_info_raw, "notoken")

        assert user_info.htl_class == "1AHWII"
        assert user_info.htl_division == "WII"
        assert user_info.username == "A Name"
        assert user_info.id == "notoken"
        assert user_info.user_id == "111111"

    def test_get_ui_wil(self):
        user_info_raw = {
            'count': 1, 
            '0': {'mail': {'count': 2, '0': 'email1', '1': 'email2'}, 
            '0': 'mail', 'displayname': {'count': 1, '0': 'A Name'}, 
            '1': 'displayname', 'count': 2, 'dn': 'cn=111111,ou=1AHWIL,ou=WI,o=HTBL'}
        }

        user_info = get_user_info(user_info_raw, "notoken")

        assert user_info.htl_class == "1AHWIL"
        assert user_info.htl_division == "WIL"
        assert user_info.username == "A Name"
        assert user_info.id == "notoken"
        assert user_info.user_id == "111111"

    def test_get_ui_l(self):
        user_info_raw = {
            'count': 1, 
            '0': {'mail': {'count': 2, '0': 'email1', '1': 'email2'}, 
            '0': 'mail', 'displayname': {'count': 1, '0': 'A Name'}, 
            '1': 'displayname', 'count': 2, 'dn': 'cn=111111,ou=1AHWII,ou=L,o=HTBL'}
        }

        user_info = get_user_info(user_info_raw, "notoken")

        assert user_info == None