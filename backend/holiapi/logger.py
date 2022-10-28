import datetime
from holiapi.config import config, PATH

LOG_FILE = f"{PATH}/log.txt"

def log(info: str):
    if not config.log:
        return
        
    with open(LOG_FILE, "a") as log_file:
        msg = f"[{datetime.datetime.now()}] {info}\n"
        log_file.write(msg)