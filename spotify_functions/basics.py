"""
This programs holds functions that are used through out the other modules in the folder.
Author: Kevin Land
"""
import requests
from appCredentials import USERNAME

# This class assists the creation of a token from the home page of the app
# token code is the auth token needed for most spotify requests
class AuthToken:
    __slots__ = ["__auth_token", "__refresh_token", "__profile", "__token"]

    def __init__(self, auth_token, refresh_token="", profile=""):
        self.__auth_token = auth_token
        self.__refresh_token = refresh_token
        self.__profile = profile
        self.__token = self.__auth_token
    
    # Returns auth token until the user refreshes a token
    def get_token(self):
        return self.__token
    
    # Takes the auth API call response and sets tokens
    def set_tokens(self, resp):
        self.__auth_token = resp["access_token"]
        self.__refresh_token = resp["refresh_token"]
        self.__token = resp["access_token"]
    
    def get_new_refresh(self):
        response = requests.post(
            "https://accounts.spotify.com/api/token",
            data={
                "grant_type": "refresh_token",
                "refresh_token":self.__refresh_token,
                "client_id":USERNAME
            },
            headers={
                "Content-Type":"application/x-www-form-urlencoded"
            }
        )
        self.__auth_token = response.json()["access_token"]
        self.__refresh_token = response.json()["refresh_token"]
        self.__token = self.__refresh_token

TOKEN = AuthToken("")

# space = %20
# , = %2C
# : = %3A
def replace_char(a_string, replacement):
    new_string = ""
    for char in a_string:
        if(char == " "):
            new_string += replacement
        else:
            new_string += char
    return new_string

# Formats optional fields so that it can be added to the end of a base URL.
def add_URL_items(title_list, value_list):
    extension = "?"
    for index in range(len(title_list)):
        if(value_list[index] == None):
            continue
        else:
            extension += "&" + title_list[index] + "=" + str(value_list[index])
    return extension[0:1] + extension[2:]

# Every function uses HTTP get. This helper function just saves space.
def requesting(url, func=requests.get):
    return func(url,headers={"Authorization": f"Bearer {TOKEN.get_token()}"}).json()