import hashlib
from flask import jsonify
from flask_restful import Resource, request
from flask_jwt import jwt_required, current_identity
from datetime import date
import utils
from typing import List
import os
from user import User
from pdf_save import save_imgs_from_pdf
import json
from utils import entries
from api_limiter import limiter
from logger import log
import sqlite3
from config import config

PATH = os.path.dirname(os.path.realpath(__file__))
USER_DB = f"{PATH}/db/user_database.db"
PDF_LOGO_PATH = "logos/pdf_logo/pdf.png"

MISSING_FILE = "Es wurde keine Datei ausgewählt."
MISSING_TITLE = "Der Titel wurde nicht angegeben."
MISSING_TAGS = "Tags müssen noch hinzugefügt werden."
SUCCESSFUL_UPLOAD = "Upload wurde erfolgreich durchgeführt."

def add_upload_id_to_db(upload_id: int, user: User):
    con = sqlite3.connect(USER_DB)
    cur = con.cursor()

    user.id["uploaded"].append(upload_id)

    updated_uids = {
        "uploaded": user.id["uploaded"],
        "fav": user.id["favs"]
    }

    cur.execute("update users set entry_info = ? where user_id=?", (json.dumps(updated_uids), user.id["user_id"]))
    con.commit()
    con.close()

class UploadMsgs():
    missing_file = ""
    missing_title = ""
    erroneous_date = ""
    missing_tags = ""
    no_user_terms = ""
    successful_upload = ""


    def has_errors(self) -> bool:
        return self.missing_file != "" or self.missing_title != "" or self.erroneous_date != "" \
            or self.missing_tags != "" or self.no_user_terms != ""

    def as_json(self):
        return {"missing_file": self.missing_file, 
                    "missing_title": self.missing_title,
                    "erroneous_date": self.erroneous_date,
                    "missing_tags": self.missing_tags,
                    "no_user_terms": self.no_user_terms,
                    "successful_upload": self.successful_upload
    }

def file_ext(file_name: str):
    splitted_ct = file_name.split(".")
    data_type = splitted_ct[len(splitted_ct)-1]
    return data_type

accepted_exts = ["pdf", "rs", "java", "py", "js", "cpp", "c"]

def is_ext_accepted(ext: str) -> bool:
    return ext in accepted_exts
    

class FileDetails:
    def __init__(self, file_name: str, data: List[int]):
        self.file_name = file_name
        self.ext = file_ext(file_name)
        self.data = bytearray(data)
        self.hash = hashlib.sha256(self.data).hexdigest()[0:32]
        self.save_path = f"{PATH}/static/files/{self.hash}.{self.ext}"

    def save_to_disk(self):
        with open(self.save_path, "wb") as f:
            f.write(self.data)

def save_upload_dict_as_json(upload_info, uid: int):
    with open(f"{PATH}/static/uploaded/{uid}.json", mode="w") as file:
        global entries
        entries[uid] = upload_info

        # move newest entry to the start
        # there is a better way to implement this
        entries = dict(sorted(entries.items(), key=utils.sorting, reverse=True))
        #entries.insert(0, upload_info)

        json.dump(upload_info, file)

class UploadDetails:
    def __init__(self, file: FileDetails, title: str, date: str, tags: str, user: User):
        self.file = file
        self.img_exts = []
        
        self.title = title
        self.date = date

        split_tags = tags.split()
        split_tags.append(date)
        # append division of user
        split_tags.append(user.id["htl_division"])
        self.tags = split_tags

        self.uploader = user.id["user_id"]
        
        self.uid = config.total_uploads
        config.total_uploads += 1
        config.save()

        self.view = ""

    # TODO
    def save_prog(self):
        self.view = utils.get_proglogo_from_file_type(self.file.ext)
        if self.file.ext not in self.tags:
            self.tags.append(self.file.ext)
         
    def save_pdfs(self):
        self.img_exts = save_imgs_from_pdf(self.file.save_path, self.file.hash)
        
        if not self.img_exts:
            self.view = PDF_LOGO_PATH

        if "pdf" not in self.tags:
            self.tags.append("pdf")

    def save_to_disk(self):
        self.file.save_to_disk()
        
        upload_type = "pdf"

        if self.file.ext == "pdf":
            self.save_pdfs()
        # this works as only pdfs and some source files can be selected
        else:
            self.save_prog()
            upload_type = "prog"

        upload_info = {
            "uid": self.uid,
            "title": self.title,
            "date": self.date,
            "tags": self.tags,
            "view": self.view,
            "img_exts": self.img_exts,
            "usid": self.uploader,
            "ut": upload_type,
            "ext": self.file.ext,
            "hash": self.file.hash
        }
        save_upload_dict_as_json(upload_info, self.uid)
    
class Upload(Resource):
    decorators = [jwt_required(), limiter.limit("3/second")]
    def post(self):
        msg = UploadMsgs()

        today = str(date.today())

        year, month, day  = today[0:4], today[5:7], today[8:10]
        today = day + "." + month + "." + year

        json_data = request.get_json(force=True)
        
        file = json_data["file"]
        file_name = file["name"]
        file_data = file["data"]

        title = json_data["title"]
        returned_date = json_data["date"]
        tags = json_data["tags"]


        if file_name == "" or file_data == []:
            msg.missing_file = MISSING_FILE
        elif not is_ext_accepted(file_ext(file_name)):
            msg.missing_file = "Diese Dateiart wird nicht unterstützt! Unterstützte Dateien: .pdf, .rs, .java, .py, .js, .c, .cpp"

        if title == "":
            msg.missing_title = MISSING_TITLE

        if tags == "":
            msg.missing_tags = MISSING_TAGS

        (current_date, date_error) = utils.check_date(today, returned_date)
        msg.erroneous_date = date_error
        
        # check date errors
        if msg.has_errors():
            return msg.as_json()

        file = FileDetails(file_name, file_data)

        upload = UploadDetails(
            file, title, current_date, 
            tags, current_identity
        )

        self.handle_upload(upload)
        msg.successful_upload = SUCCESSFUL_UPLOAD

        log(f"{upload.uploader}/{current_identity.id['username']}/{current_identity.id['htl_class']} uploaded entry called '{title}' with tags '{tags}' and hash '{file.hash}/{upload.uid}'.")
        add_upload_id_to_db(upload.uid, current_identity)
        print(f"added to own ups?: {current_identity.id['uploaded']}")

        return msg.as_json()
    
    def handle_upload(self, upload: UploadDetails):
        upload.save_to_disk()
        pass
        