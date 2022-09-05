"""
This programs goal is to use all profile modifications available for Spotify Developers.
Author: Kevin Land
"""
from spotify_functions.basics import *

BASE_URL = {"taa":"https://api.spotify.com/v1/audio-analysis/", 
"taf":"https://api.spotify.com/v1/audio-features",
"t":"https://api.spotify.com/v1/tracks",
}

# Requires an auth key only
def get_track_audio_analysis(track_id):
    return requesting(f"{BASE_URL['taa']}{track_id}")

# Requires an auth key only
def get_track_audio_features(track_id):
    return requesting(f"{BASE_URL['taf']}/{track_id}")

# Requires an auth key only
def get_multiple_tracks_audio_features(track_ids:list):
    extension = "?ids="
    for id in track_ids:
        extension += "," + id
    extension = replace_char(extension[:5] + extension[6:], "%2C")
    return requesting(f"{BASE_URL['taf']}{extension}")

# Requires an auth key only
def get_track(track_id, market=None):
    return requesting(f"{BASE_URL['t']}/{track_id}{add_URL_items(['market'], [market])}")

# Requires an auth key only
# Maximum 50 tracks
def get_multiple_tracks(track_ids:list, market=None):
    id_string = ""
    for id in track_ids:
        id_string += "," + id
    id_string = replace_char(id_string[1:], "%2C")
    return requesting(f"{BASE_URL['t']}{add_URL_items(['market', 'ids'], [market, id_string])}")

def main():
    # print(get_track_audio_analysis("47BBI51FKFwOMlIiX6m8ya"))
    # print(get_track_audio_features("2tUBqZG2AbRi7Q0BIrVrEj"))
    # print(get_multiple_tracks_audio_features(["7ouMYWpwJ422jRcDASZB7P",  "4VqPOruhp5EdPBeR92t6lQ", "2takcwOaAZWiXQijPHIx7B"]))
    print(get_track("6IpfP2y5bzcxAJUFfHtlX5", "US"))
    # print(get_multiple_tracks(["47BBI51FKFwOMlIiX6m8ya", "4X4v3KtkUXwXvDBw5KS9cp"], "US"))
    pass



if __name__ == "__main__":
    main()