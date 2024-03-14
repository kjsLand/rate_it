"""
This programs holds functions that are used through out the other modules in the folder.
Author: Kevin Land
"""
from tkinter import END, Button
import requests

# This class assists the creation of a token from the home page of the app
# token code is the auth token needed for most spotify requests
class AuthToken:
    __slots__ = ["__token_code", "__profile"]

    def __init__(self, token_code):
        self.__token_code = token_code
    
    def get_token(self):
        return self.__token_code
    
    def get_profile(self):
        return self.__profile

    def set_token(self, spotify, token_url, text_feild, auth, frame, func):
        response = text_feild.get("1.0", END)
        response = response[:len(response)-1]
        token = spotify.fetch_token(token_url, auth=auth, authorization_response=response)

        # Fetch a protected resource, i.e. user profile
        self.__profile = spotify.get('https://api.spotify.com/v1/me')

        print(token)

        # Gets Authorization Token
        self.__token_code = token["access_token"]

        # Clears frame
        for widget in frame.winfo_children():
            widget.destroy()

        Button(frame, text="Click Here Before You Start", command=lambda:func(frame)).grid(row=0, column=0)

TOKEN = AuthToken("BQDkVU0Dgty0UMfs9kzrAiDVS4hWGT3x_hpB-BZVwNWKS_imJx6fuHUOTnwkMMFdZJt4g6e9CdbBKO4xb7W8IH9Kb9CGd3dfLrBWgWvCQjDFYrYH6vpvmZgZGeqSfqHkNOr0uW4CM7u1j9GDLIz8vgS2dKmnEBc1FxTCWjSK7FczeU9c0mYkKn99p6nv62KTx2ssqKv4M5hGt9k8DWjohqCycvDVSa4eDlJKgST5ZaBvam0q_P0nFOf6tAkVk5Qy3u9dYgUS9Q")

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