"""
This programs goal is to use all player modifications available for Spotify Developers.
Author: Kevin Land
"""
from spotify_function.basics import *

# Requires user-read-recently-played
def get_recently_played(limit=None):
    return requesting("https://api.spotify.com/v1/me/player/recently-played" + add_URL_items(["limit"], [limit]))

# Requires user-read-playback-state
def get_playback_state(market=None, additional_types=None):
    extension = add_URL_items(["market", "additional_types"], [market, additional_types])
    return requesting("https://api.spotify.com/v1/me/player" + extension)

# Requires user-read-playback-state
def get_available_devices():
    return requesting("https://api.spotify.com/v1/me/player/devices")

# Requires user-read-currently-playing
def get_current_play(market=None, additional_types=None):
    extension = add_URL_items(["market", "additional_types"], [market, additional_types])
    return requesting("https://api.spotify.com/v1/me/player/currently-playing" + extension)

# Requires user-modify-playback-state
# Action options: next, previous, pause
# pause needs more testing
def playback_action(device_id, action):
    return requesting(f"https://api.spotify.com/v1/me/player/{action}?device_id={device_id}", requests.post)

# Requires user-modify-playback-state
def add_to_queue(uri, device_id=None):
    extension = add_URL_items(["uri", "device_id"], [uri, device_id])
    return requesting("https://api.spotify.com/v1/me/player/queue" + extension, requests.post)

# Requires user-modify-playback-state
# Needs more optimization
def resume(song, device_id=None):
    response = requests.put(
        "https://api.spotify.com/v1/me/player/play" + add_URL_items(["device_id"], [device_id]), 
        headers={
            "Authorization": f"Bearer {TOKEN.get_token()}"
        }, 
        json = {
            "context_uri": song,
            "offset": {
                "position": 5
                },
            "position_ms": 0
        }).json()
    return response

# Requires user-modify-playback-state
# modes: repeat, shuffle
# States: Track, context, off (repeat) / True, False (shuffle)
def playback_mode(mode, state, device_id=None):
    extension = add_URL_items(["state", "device_id"], [state, device_id])
    return requesting(f"https://api.spotify.com/v1/me/player/{mode+extension}", requests.put)

# Requires user-modify-playback-state
def seek_to(position_ms:int, device_id=None):
    extension = add_URL_items(["position_ms", "device_id"], [position_ms, device_id])
    return requesting("https://api.spotify.com/v1/me/player/seek" + extension, requests.put)

# Requires user-modify-playback-state
def transfer_playback(device_ids:list):
    response = requests.put(
        "https://api.spotify.com/v1/me/player",
        headers={
            "Authorization": f"Bearer {TOKEN.get_token()}"
        },
        json = {
            "device_ids": device_ids
        }).json()
    return response

# Requires user-modify-playback-state
# volume is in percent
def set_volume(volume_percent, device_id=None):
    extension = add_URL_items(["volume_percent", "device_id"], [volume_percent, device_id])
    return requesting("https://api.spotify.com/v1/me/player/volume" + extension, requests.put)

def main():
    # print(get_recently_played())
    # print(get_playback_state())
    # print(get_available_devices())
    # print(get_current_play())
    # print(playback_action("ae781267dcbfd24d2b20d939c352e35dd9fe2b6e", "next"))
    # print(playback_action("ae781267dcbfd24d2b20d939c352e35dd9fe2b6e", "previous"))
    # print(playback_action("ae781267dcbfd24d2b20d939c352e35dd9fe2b6e", "pause"))
    print(add_to_queue("spotify:album:6pOiDiuDQqrmo5DbG0ZubR", "ae781267dcbfd24d2b20d939c352e35dd9fe2b6e"))
    # print(resume("ae781267dcbfd24d2b20d939c352e35dd9fe2b6e"))
    # print(playback_mode("repeat", "off", "ae781267dcbfd24d2b20d939c352e35dd9fe2b6e"))
    # print(playback_mode("shuffle", "True", "ae781267dcbfd24d2b20d939c352e35dd9fe2b6e"))
    # print(seek_to(240, "ae781267dcbfd24d2b20d939c352e35dd9fe2b6e"))
    # print(transfer_playback(["ae781267dcbfd24d2b20d939c352e35dd9fe2b6e", "ae781267dcbfd24d2b20d939c352e35dd9fe2b6e"]))
    # print(set_volume(90, "ae781267dcbfd24d2b20d939c352e35dd9fe2b6e"))
    pass

if __name__ == "__main__":
    main()