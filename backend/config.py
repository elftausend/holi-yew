import os
import json
from typing import List

PATH = os.path.dirname(os.path.realpath(__file__))
CONFIG_PATH = f"{PATH}/config.json"

class Config:
    def __init__(self, banned_ids: List[str], admin_ids: List[str], log: bool):
        self.banned_ids = banned_ids
        self.admin_ids = admin_ids
        self.log = log
    
def config_file_setup():
    if not os.path.exists(CONFIG_PATH):
        with open(CONFIG_PATH, "w") as config_file:
            json.dump({
                "banned_ids": [],
                "admin_ids": [],
                "log": True
            }, config_file, indent=4)

    with open(CONFIG_PATH, "r") as config_file:
        config_data = json.load(config_file)
        return Config(
            config_data["banned_ids"],
            config_data["admin_ids"],
            config_data["log"]
        )

config = config_file_setup()
    