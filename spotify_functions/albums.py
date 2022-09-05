"""
This program uses all album actions available for a Spotify Developer.
Author: Kevin Land
"""
from spotify_functions.basics import *

BASE_URL = "https://api.spotify.com/v1/albums/"

# Requires an auth key only
def get_album_tracks(album_id, market=None, limit=None, offset=None):
    extension = album_id + "/tracks" + add_URL_items(["market", "limit", "offset"], [market, limit, offset])
    return requesting(BASE_URL + extension)

# Requires an auth key only
def get_album(album_id, market):
    extension = album_id + add_URL_items(["market"], [market])
    return requesting(BASE_URL + extension)

# Requires an auth key only
# Maximum 20 ids
def get_multiple_albums(album_id:list, market):
    extension = "?ids="
    for id in album_id:
        extension += "%2C" + id
    extension += "&market=" + market
    return requesting(BASE_URL[:len(BASE_URL)-1] + extension[0:5] + extension[8:])

def main():
    # print(get_album_tracks("4aawyAB9vmqN3uQ7FjRGTy", "US", 3, 0))
    # print(get_album("4aawyAB9vmqN3uQ7FjRGTy", "US"))
    # print(get_multiple_albums(["382ObEPsp2rxGrnsizN5TX", "1A2GTWGtFfWp7KSQTwWOyo", "2noRn2Aes5aoNVsU6iWThc"], "ES"))
    pass

if __name__ == "__main__":
    main()