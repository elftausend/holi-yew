import hashlib
from flask import jsonify
from flask_restful import Resource, request
from flask_jwt import jwt_required, current_identity

from typing import List


class UploadError():
    missing_file = ""
    missing_title = ""
    erroneous_date = ""
    missing_tags = ""
    no_user_terms = ""


    def has_errors(self) -> bool:
        return self.missing_file != "" or self.missing_title != "" or self.erroneous_date != "" \
            or self.missing_tags != "" or self.no_user_terms != ""

    def as_json(self):
        return jsonify({"missing_file": self.missing_file, 
                    "missing_title": self.missing_title,
                    "erroneous_date": self.erroneous_date,
                    "missing_tags": self.missing_tags,
                    "no_user_terms": self.no_user_terms
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
    
    def data_hash(self) -> str:
        return hashlib.sha256(bytearray(self.data)).hexdigest()

    def save_to_disk(self):
        pass

class UploadDetails:
    def __init__(self, file: FileDetails, title: str, date: str, tags: str, uploader: str):
        self.file = file
        self.hash = file.data_hash()
        
        self.title = title
        self.date = date
        self.tags = tags
        self.uploader = uploader
        

    def save_to_disk(self):
        pass

class Upload(Resource):
    @jwt_required()
    def post(self):
        error = UploadError()
        json_data = request.get_json(force=True)
        
        file = json_data["file"]
        file_name = file["name"]
        file_data = file["data"]

        title = json_data["title"]
        date = json_data["date"]
        tags = json_data["tags"]

        if file_name == "" or file_data == []:
            error.missing_file="Es wurde keine Datei ausgewählt.";

        if title == "":
            error.missing_title="Der Titel wurde nicht angegeben."

        if tags == "":
            error.missing_tags="Die Tags müssen noch hinzugefügt werden."

        # check date errors

        if error.has_errors():
            return error.as_json()

        file = FileDetails(file_name, file_data)

        upload = UploadDetails(
            file, title, date, 
            tags, str(current_identity.id)
        )

        self.handle_upload(upload)
        
    def handle_upload(self, upload: UploadDetails):

        pass
        