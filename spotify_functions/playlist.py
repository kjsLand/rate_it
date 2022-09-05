"""
This programs goal is to use all playlist modifications available for Spotify Developers.
Author: Kevin Land
"""
from spotify_functions.basics import *

# Requires playlist-read-private
def get_current_playlists():
    return requesting("https://api.spotify.com/v1/me/playlists")

# No specific auth is required, but key is needed in headers
def get_cover_image(playlist_id):
    return requesting(f"https://api.spotify.com/v1/playlists/{playlist_id}/images")

# Requires playlist-read-private and playlist-read-public
def get_playlists_items(playlist_id):
    return requesting(f"https://api.spotify.com/v1/playlists/{playlist_id}/tracks")

# No specific auth is required, but key is needed in headers
def get_playlist(playlist_id):
    return requesting(f"https://api.spotify.com/v1/playlists/{playlist_id}")

# Requires playlist-read-private and playlist-read-collaborative
def get_user_playlists(user_id):
    return requesting(f"https://api.spotify.com/v1/users/{user_id}/playlists")

# Requires playlist-modify-public and playlist-modify-private
# Alter requesting in basics to take in a json parameter
def create_playlist(username:str, name:str, public:bool, description:str):
    return requests.post(f"https://api.spotify.com/v1/users/{username}/playlists",
    headers={"Authorization": f"Bearer {TOKEN.get_token()}"},
    json = {
            "name": name,
            "public": public,
            "description": description
        }
    ).json()

# Requires playlist-modify-public and playlist-modify-private
# adds songs that arent tracks
# Maximum of 100 songs can be added
def add_playlist_items(playlist_id, track_ids:list):
    url = f"https://api.spotify.com/v1/playlists/{playlist_id}/tracks"
    uris_beginning = replace_char('spotify:track:', '%3A')
    extension = ""
    for id in track_ids:
        extension += "," + uris_beginning + id
    extension = replace_char(extension[1:], "%2C")
    return requesting(f"{url}{add_URL_items(['uris'],[extension])}", requests.post)

# Requires playlist-modify-public and playlist-modify-private
# Still need to test
def remove_playlist_items(playlist_id):
    return requesting("https://api.spotify.com/v1/playlists/{playlist_id}/tracks", requests.delete)

# Requires playlist-modify-public and playlist-modify-private
# Still needs testing
def update_playlist_items(playlist):
    id = playlist["id"]
    response = requests.put("https://api.spotify.com/v1/playlists/"+ id +"/tracks",
    headers={"Authorization": f"Bearer {TOKEN.get_token()}"})
    json_resp = response.json()
    return json_resp