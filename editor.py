import json

LOCAL_DATABASE = "./rate.txt"

def addSong(name, rating, song_id, artist):
    lines = open(LOCAL_DATABASE).readlines()
    if len(lines) == 0: # First time adding a song
        contents = {}
    else:
        contents:dict = json.loads(open(LOCAL_DATABASE).readlines()[0])
    
    if artist not in contents.keys():
        contents[artist] = {}
    contents[artist][song_id] = {"rating":rating,"name":name}

    with open(LOCAL_DATABASE, "w") as file:
        file.write(json.dumps(contents))

# Removes a song from the local database
def removeSong(song_id, artist):
    contents:dict = json.loads(open(LOCAL_DATABASE).readlines()[0])
    contents[artist].pop(song_id)
    with open(LOCAL_DATABASE, "w") as file:
        file.write(json.dumps(contents))

# Returns the song if rated, false otherwise
def isRated(song_id, artist_name):
    contents:dict = json.loads(open(LOCAL_DATABASE).readlines()[0])
    if artist_name not in contents:
        return False
    elif song_id not in contents[artist_name]:
        return False
    return contents[artist_name][song_id]

def getAllArtists():
    contents:dict = json.loads(open(LOCAL_DATABASE).readlines()[0])
    return contents.keys()

# type options: rating, name, id, none gives all info
def getRatedSongs(artist_name, rating=0, type=None):
    tracks = []
    contents:dict = json.loads(open(LOCAL_DATABASE).readlines()[0])

    if artist_name not in contents.keys(): # When an artist isn't found
        return tracks
    
    songs:dict = contents[artist_name]
    for sid in songs.keys():
        if songs[sid]["rating"] >= rating: # above the given rating
            if type == "id":
                tracks.append(sid)
            elif type == "rating":
                tracks.append(songs[sid]["rating"])
            elif type == "name":
                tracks.append(songs[sid]["name"])
            elif not type:
                tracks.append((songs[sid]["name"],songs[sid]["rating"],sid))
    return tracks

def getOverallRating(artist_name):
    all_songs = getRatedSongs(artist_name, type="rating")
    total = 0
    for num in all_songs:
        total += num

    if len(all_songs) == 0:
        return 0
    return (total/len(all_songs)).__round__(1)
    
def getTopRated(artist_name):
    all_songs = getRatedSongs(artist_name)
    all_songs.sort(key=lambda x: x[1], reverse=True)
    return all_songs[:5]