"""
This program makes the browse_basics module more useful.
Author: Kevin Land
"""
from spotify_functions.basics import *

BASE_URL = "https://api.spotify.com/v1/artists/"

# Requires an auth key only
def get_artist_albums(artist_id, include_groups=None, market=None, limit=None, offset=None):
    extension = artist_id + "/albums" + add_URL_items(["include-groups", "market", "limit", "offset"], [include_groups, market, limit, offset])
    return requesting(BASE_URL + extension)

# Requires an auth key only
# Gives 20 relates artists
def get_related_artist(artist_id):
    return requesting(BASE_URL + artist_id + "/related-artists")

# Requires an auth key only
def get_artist_top_tracks(artist_id, market="US"):
    return requesting(BASE_URL + artist_id + "/top-tracks" + add_URL_items(["market"], [market]))

# Requires an auth key only
def get_artist(artist_id:str):
    return requesting(BASE_URL + artist_id)

# Requires an auth key only
def get_multiple_artists(artist_id:list):
    extension = "?ids="
    for id in artist_id:
        extension += "%2C" + id
    return requesting(BASE_URL[:len(BASE_URL)-1] + extension[0:5] + extension[8:])