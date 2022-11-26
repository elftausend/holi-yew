import os
import datetime
from flask import Flask
from flask_restful import Api
from flask_cors import CORS
#import jwt

from holiapi.flagsys.update_flags import FlagUpdate
from holiapi.flagsys.users import Users
from holiapi.entries.entries import Entries, Entry, EntryCount
from holiapi.entries.edit import EditEntries, EditEntry
from holiapi.entries.delete import DeleteEntry
from holiapi.tags.unique_tags import UniqueTags
from holiapi.auth import Auth, jwt
from holiapi.upload import *
from holiapi.user import *
from holiapi.config import create_dirs
from holiapi.favo.favo import Favo
from holiapi.favo.unfavo import UnFavo

from holiapi.api_limiter import limiter


app = Flask(__name__)
jwt.init_app(app)

def init_and_run(host: str):
    create_dirs()

    # User database init
    # db.init_app(app)

    # Limiter init
    limiter.init_app(app)

    api = Api(app)
    app.secret_key = os.urandom(32)

    # can be disabled for HTL HL (due to trunk proxy config)
    CORS(app)
    #app.config['CORS_HEADERS'] = 'Content-Type'

    #app.config["SQLALCHEMY_DATABASE_URI"] = "sqlite:///user.db"
    app.config["JWT_SECRET_KEY"] = os.urandom(32)
    app.config["JWT_ACCESS_TOKEN_EXPIRES"] = datetime.timedelta(minutes=24*60*7)
    
    #app.config["JWT_EXPIRATION_DELTA"] = datetime.timedelta(minutes=24*60*7)
    #app.config["JWT_AUTH_PASSWORD_KEY"] = "code"
    #app.config["JWT_AUTH_URL_RULE"] = "/api/auth"


    api.add_resource(Auth, "/auth")
    api.add_resource(UserRoute, '/user')
    api.add_resource(Entries, '/entries')
    api.add_resource(EntryCount, '/entry_count')
    api.add_resource(Entry, '/entry/<int:uid>')
    api.add_resource(Upload, '/upload')
    api.add_resource(EditEntries, "/editable_entries")
    api.add_resource(EditEntry, "/edit_entry")
    api.add_resource(DeleteEntry, "/delete")
    api.add_resource(UniqueTags, "/unique_tags")
    api.add_resource(Users, "/users")
    api.add_resource(FlagUpdate, "/incr_flag")
    api.add_resource(Favo, "/favo")
    api.add_resource(UnFavo, "/unfavo")

    from holiapi.db.setup_db import create_user_db
    create_user_db()
    #from waitress import serve
    #serve(app, host=host, port=82, threads=16)
    app.run(host=host, debug=True, port=82)




#if __name__ == '__main__':
    