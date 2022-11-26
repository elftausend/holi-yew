import hashlib
from flask_restful import Resource, request
from flask_jwt_extended import jwt_required, current_user
from datetime import date
from typing import List, Tuple
from holiapi.user import User
import json
from holiapi import utils
from holiapi.pdf_save import save_imgs_from_pdf
from holiapi.api_limiter import limiter
from holiapi.logger import log
from holiapi.config import config, PATH
from holiapi.db.entry_info import write_entry_info_to_db

PDF_LOGO_PATH = "logos/pdf_logo/pdf.png"

MISSING_FILE = "Es wurde keine Datei ausgewählt."
MISSING_TITLE = "Der Titel wurde nicht angegeben."
MISSING_TAGS = "Tags müssen noch hinzugefügt werden."
SUCCESSFUL_UPLOAD = "Upload wurde erfolgreich durchgeführt."
ALREADY_UPLOADED = "Dieses File wurde schon hochgeladen."
ERRORNEOUS_DIVISION = "Ungültige Abteilung -> (ET, IT, ME, MB, WIL, WII)"

def add_upload_id_to_db(upload_id: int, user: User):
    user.uploaded.append(upload_id)

    entry_info = {
        "uploaded": user.uploaded,
        "fav": user.favs
    }

    write_entry_info_to_db(user.user_id, entry_info)

class UploadMsgs():
    missing_file = ""
    missing_title = ""
    erroneous_date = ""
    missing_tags = ""
    no_user_terms = ""
    successful_upload = ""
    erroneous_division = ""

    def has_errors(self) -> bool:
        return self.missing_file != "" or self.missing_title != "" or self.erroneous_date != "" \
            or self.missing_tags != "" or self.no_user_terms != "" or self.erroneous_division != ""

    def as_json(self):
        return {"missing_file": self.missing_file, 
                    "missing_title": self.missing_title,
                    "erroneous_date": self.erroneous_date,
                    "missing_tags": self.missing_tags,
                    "no_user_terms": self.no_user_terms,
                    "successful_upload": self.successful_upload,
                    "erroneous_division": self.erroneous_division
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

def sort_by_id(entry):
    return entry[1]["uid"]

def save_upload_dict_as_json(upload_info, uid: int):
    with open(f"{PATH}/static/uploaded/{uid}.json", mode="w") as file:
        utils.entries[uid] = upload_info

        # ordering is done by the frontend
        #utils.entries = dict(sorted(utils.entries.items(), key=sort_by_id, reverse=True))
        #entries.insert(0, upload_info)

        json.dump(upload_info, file)

class UploadDetails:
    def __init__(self, file: FileDetails, title: str, date: str, tags: str, user: User, htl_division: str):
        self.file = file
        self.img_exts = []
        
        self.title = title
        self.date = date

        split_tags = tags.split()
        split_tags.append(date)
        # append division of user
        split_tags.append(htl_division)
        self.tags = split_tags

        self.uploader = user.user_id
        
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
            "favs": 0,
            "hash": self.file.hash
        }
        save_upload_dict_as_json(upload_info, self.uid)

def division_exist(htl_division: str) -> bool:
    if htl_division == "ET":
        return True
    elif htl_division == "IT":
        return True
    elif htl_division == "ME":
        return True
    elif htl_division == "MB":
        return True
    elif htl_division == "WIL":
        return True
    elif htl_division == "WII":
        return True

    return False

def check_division(htl_division: str, user: User) -> Tuple[str, str]:
    if not htl_division:
        return (user.htl_division, "")
    
    # TODO: mind lebensmittel
    if not division_exist(htl_division):
        return (htl_division, ERRORNEOUS_DIVISION)

    return (htl_division, "")

class Upload(Resource):
    decorators = [jwt_required(), limiter.limit("3/second")]
    def post(self):
        if current_user.upload_banned:
            return
            
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

        htl_division = json_data["htl_division"]

        (htl_division, division_error) = check_division(htl_division, current_user)
        msg.erroneous_division = division_error


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

        if msg.has_errors():
            return msg.as_json()
        
        file = FileDetails(file_name, file_data)
        
        if utils.is_hash_in_file(file.hash):
            msg.missing_file = ALREADY_UPLOADED
            return msg.as_json()
    
        upload = UploadDetails(
            file, title, current_date, 
            tags, current_user, htl_division
        )

        self.handle_upload(upload)
        msg.successful_upload = SUCCESSFUL_UPLOAD

        log(f"{upload.uploader}/{current_user.username}/{current_user.htl_class} uploaded entry called '{title}' with tags '{tags}' and hash '{file.hash}/{upload.uid}'.")
        add_upload_id_to_db(upload.uid, current_user)
        print(f"added to own ups?: {current_user.uploaded}")

        return msg.as_json()
    
    def handle_upload(self, upload: UploadDetails):
        upload.save_to_disk()
        