import os
import json
from typing import List

PATH = os.path.dirname(os.path.realpath(__file__))
CONFIG_PATH = f"{PATH}/config.json"

class Config:
    def __init__(self, banned_ids: List[str], admin_ids: List[str], whitelist_ids: List[str], whitelist: bool, log: bool, total_uploads):
        self.banned_ids = banned_ids
        self.admin_ids = admin_ids
        self.whitelist_ids = whitelist_ids
        self.whitelist = whitelist
        self.log = log
        self.total_uploads = total_uploads

    def save(self):
        with open(CONFIG_PATH, "w") as config_file:
            json.dump({
                "banned_ids": self.banned_ids,
                "admin_ids": self.admin_ids,
                "whitelist_ids": self.whitelist_ids,
                "whitelist": self.whitelist,
                "log": self.log,
                "total_uploads": self.total_uploads
            }, config_file, indent=4)
        
def count_uploads() -> int:
    return len(os.listdir(f"{PATH}/static/uploaded/"))
    
def config_file_setup():
    if not os.path.exists(CONFIG_PATH):
        with open(CONFIG_PATH, "w") as config_file:
            json.dump({
                "banned_ids": [],
                "admin_ids": [],
                "whitelist_ids": [],
                "whitelist": False,
                "log": True,
                "total_uploads": count_uploads()
            }, config_file, indent=4)

    with open(CONFIG_PATH, "r") as config_file:
        config_data = json.load(config_file)
        return Config(
            config_data["banned_ids"],
            config_data["admin_ids"],
            config_data["whitelist_ids"],
            config_data["whitelist"],
            config_data["log"],
            config_data["total_uploads"],
        )

config = config_file_setup()
    