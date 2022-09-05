"""
This programs goal is to use all profile modifications available for Spotify Developers.
Author: Kevin Land
"""
from spotify_functions.basics import *

BASE_URL = {"c":"https://api.spotify.com/v1/me", "u":"https://api.spotify.com/v1/users/"}

#requires user-read-private and user-read-email
def get_current_user_profile():
    return requesting(BASE_URL["c"])

def get_user_profile(user_id):
    return requesting(BASE_URL["u"] + user_id)