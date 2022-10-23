import os
import datetime
from config import config

PATH = os.path.dirname(os.path.realpath(__file__))
LOG_FILE = f"{PATH}/log.txt"

def log(info: str):
    if not config.log:
        return
        
    with open(LOG_FILE, "a") as log_file:
        msg = f"[{datetime.datetime.now()}] {info}\n"
        log_file.write(msg)