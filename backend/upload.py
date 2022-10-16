import hashlib
from flask import jsonify
from flask_restful import Resource, request
from flask_jwt import jwt_required, current_identity
from datetime import date
import utils
from typing import List
import os
from user import User
import json

PATH = os.path.dirname(os.path.realpath(__file__))

MISSING_FILE = "Es wurde keine Datei ausgewählt."
MISSING_TITLE = "Der Titel wurde nicht angegeben."
MISSING_TAGS = "Die Tags müssen noch hinzugefügt werden."
SUCCESSFUL_UPLOAD = "Upload wurde erfolgreich durchgeführt."

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
        return jsonify({"missing_file": self.missing_file, 
                    "missing_title": self.missing_title,
                    "erroneous_date": self.erroneous_date,
                    "missing_tags": self.missing_tags,
                    "no_user_terms": self.no_user_terms,
                    "successful_upload": self.successful_upload
    })

def file_ext(file_name: str):
    splitted_ct = file_name.split(".")
    data_type = splitted_ct[1]
    return data_type

class FileDetails:
    def __init__(self, file_name: str, data: List[int]):
        self.file_name = file_name
        self.ext = file_ext(file_name)
        self.data = data
        self.hash = hashlib.sha256(bytearray(self.data)).hexdigest()[0:32]
        self.save_path = f"{PATH}/static/files/{self.hash}.{self.ext}"

    def save_to_disk(self):
        with open(self.save_path, "wb") as f:
            f.write(self.data)

class UploadDetails:
    def __init__(self, file: FileDetails, title: str, date: str, tags: str, user: User):
        self.file = file
        self.pdf_picture_count = 0
        
        self.title = title
        self.date = date

        split_tags = tags.split()
        split_tags.append(date)
        # append division of user
        # split_tags.append(user.division)

        self.tags = split_tags

        self.uploader = str(user.id)
        self.view = ""
        
    def save_pdfs(self):
        self.pdf_picture_count = utils.save_imgs_from_pdf(self.file.save_path, self.file.hash)

        if self.pdf_picture_count == 0:
            self.view = "pdf_logo/pdf.png"

        if "pdf" not in self.tags:
            self.tags.append("pdf")

    def save_to_disk(self):
        self.file.save_to_disk()
        
        upload_type = "prog"

        if self.file.ext == "pdf":
            self.save_pdfs()
            upload_type = "pdf"

        with open(f"{PATH}/static/uploaded/{self.file.hash}.json", mode="w") as file:
            upload_info = {
                "title": self.title,
                "date": self.date,
                "tags": self.tags,
                "view": self.view,
                "pdf_pc": self.pdf_picture_count,
                "usid": self.uploader,
                "ut": upload_type
            }
            json.dump(upload_info, file)
    
class Upload(Resource):
    @jwt_required()
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
        return msg.as_json()
    
    def handle_upload(self, upload: UploadDetails):
        
        pass
        