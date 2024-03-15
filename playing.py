# last four scopes are needed
from spotify_functions.player import *
from spotify_functions.playlist import add_playlist_items, create_playlist
from spotify_functions.profile import get_current_user_profile
from spotify_functions.tracks import *
from spotify_functions.artists import *
from spotify_functions.albums import *
from editor import *

def getDeviceID():
    output = get_available_devices()
    id = output["devices"][0]["id"]
    return id

# Have to be playing a song for time stuff to be added
# make a try except to avoid this.
# song parameter takes the id of the song
def songInfo(song=None):
    if TOKEN.get_token() == "":
        raise Exception("No Token - Auth account and try again")

    if(not song): # if no song is given, then the function gets the current song's info
        output = get_current_play()["item"]
    else:
        output = get_track(song) # Gets the specified track's info

    try: # Makes sure there is no error receiving the information
        print(output["error"])
    except KeyError:
        info = dict()
        info["song id"] = output["id"]
        info["song name"] = output["name"]
        info["uri"] = output["uri"]
        info["explicit"] = output["explicit"]
        info["album id"] = output["album"]["id"]
        info["album name"] = output["album"]["name"]
        info["imgurl"] = output["album"]["images"][0]["url"]

        info["artists id"] = list()
        info["artists"] = list()
        for i in range(len(output["artists"])): # index 0 will always be the main artist
                info["artists id"].append(output["artists"][i]["id"])
                info["artists"].append(output["artists"][i]["name"])

        return info

# Gets all artists tracks and albums that they made
# parameter: artist's id that is given by spotify, type is either uri or id
# returns a list of all their albums in id form
def artistAlbums(artist_id, type):
    start_point = 0
    info = list()
    total_songs = 0

    while(True):
        output = get_artist_albums(artist_id, include_groups="album,single", offset=start_point)
        
        for item in output["items"]:
            if(item["album_group"] != "appears_on"):
                info.append(item[type])
                total_songs+=(int)(item["total_tracks"])
        
        if((int)(output["total"]) < 20+start_point):
            break
        else:
            start_point+=20

    return info, total_songs

# Gets all the songs from an album
# paramters: album id, artists name, uri or id of the songs, True if unrated mode is needed
# returns songs of the album
def getAlbumSongs(album_id, artist, type, unrated_mode=False):
    output = get_album_tracks(album_id)
    songs = []

    for item in output["items"]:
        if(unrated_mode): # Adds unrated songs
                if(isRated(item["id"], artist) == False):
                    songs.append(item[type])
        else: # Adds all songs
            songs.append(item[type])
    
    return songs

# Plays all the songs that an artist has
# artist_id cand be provided, but the current artist is given by defualt.
def playDiscography(artist_id=None, artist_name=None, unrated_mode=False):
    # Grabs artist info
    if(artist_id==None and artist_name==None):
        info = songInfo()
        if(info != None):
            artist_id = info["artists id"][0]
            artist_name = info["artists"][0]

    all_songs:dict = {}
    all_albums, num_of_tracks = artistAlbums(artist_id, "id")
    print("Number of tracks: " + str(num_of_tracks) + "\nNumber of albums " + str(len(all_albums)))
    for i in range(0, len(all_albums), 20): # Splits in sections of 20
        all_songs.update(songsFromAlbums(get_multiple_albums(all_albums[i:i+20], "US"), all_songs))

    playlist_name = artist_name
    if unrated_mode:
        rated = getRatedSongs(artist_name) # Gets all rated songs
        for song in rated:
            all_songs.pop(song[0])
        playlist_name+= " (unrated)"
    
    id = makePlaylist(playlist_name)
    adding = list(all_songs.values())
    for i in range(0, len(adding), 100): # Can only request a maximum of 100 tracks
        addToPlaylist(id, adding[i:i+100])

# Puts all songs of a specified album to the user's queue
def playAlbum(id=None, artist=None):
    if(id==None):
        info = songInfo()
        id = info["album id"]
        artist = info["artists"][0]
    
    total_songs = 0
    all_songs = getAlbumSongs(id, artist, "uri")
    total_songs+=len(all_songs)
    for song in all_songs:
        try:
            add_to_queue(song) #Works but throws an error that stops the loop
        except:
            print(song + "added") #Continues the loop whiling outputing the song id
    print("Added " + str(total_songs) + " of " + artist + "'s songs to your queue")

# Plays the top ten songs of an artist
def playTopTen(artist_id=None):
    if(artist_id==None):
        info = songInfo()
        artist_id = info["artists id"][0]
        artist = info["artists"][0]

    total_songs = 0
    all_songs = get_artist_top_tracks(artist_id, "US")
    total_songs+=len(all_songs["tracks"])
    for song in all_songs["tracks"]:
        print(song["uri"])
        try:
            add_to_queue(song["uri"]) #Works but throws an error that stops the loop
        except:
            print(song["uri"] + "added") #Continues the loop whiling outputing the song id
    print("Added " + str(total_songs) + " of " + artist + "'s songs to your queue")

# Rates the song that the user is currently listening to
def rateCurSong(rating):
    song = songInfo()
    artist_name = song["artists"][0] #Gets first name if there are features

    if(isRated(song["song id"], artist_name) != False):
        print("Already rated this song (" + song["song id"] + ") by " + artist_name)
        return

    addSong(song["song name"], rating, song["song id"], artist_name)

# Changes the rating of a song that has previously been rated
def reRate(rating):
    song = songInfo()
    artist_name = song["artists"][0] #Gets first name if there are features
    removeSong(song["song id"], artist_name)
    rateCurSong(rating)

# Helper function for findFeatures
# Returns all artists that contributed to a specific set of albums
# data is a dictionary and albums_info is a get_multiple_albums request
def artistsFromAlbums(albums_info, data):
    if(data == None):
        data = dict()

    for item in albums_info["albums"]: # Gets all albums
        for all_tracks in item["tracks"]["items"]: # Gets all tracks in albums
                for person in all_tracks["artists"]: # Gets all artists in tracks
                    data[person["name"]] = person["id"] # adds them to a dict

    return data

# Helper function for playDiscography
# Returns all songs that are in a specific set of albums
# data is a dictionary and albums_info is a get_multiple_albums request
def songsFromAlbums(albums_info, data):
    if(data == None):
        data = dict()

    for item in albums_info["albums"]: # Gets all albums
        for track in item["tracks"]["items"]: # Gets all tracks in 
            data[track["name"]] = track["id"] # adds them to a dict

    return data

# This function finds all the artists that features with cur_artist
# cur_artist is the id of the artist of the song that is currently playing
def findFeatures(cur_artist=None, cur_artist_id=None):
    if(cur_artist==None):
        info = songInfo()
        cur_artist = info["artists"][0]
        cur_artist_id = info["artists id"][0]

    all_albums, num_of_tracks = artistAlbums(cur_artist_id, "id")
    print("Number of tracks: " + (str)(num_of_tracks) + "\nNumber of albums " + (str)(len(all_albums)))
    
    i = (len(all_albums)/20).__ceil__() # Can only request a maximum of 20 albums
    all_artists = dict()
    while(i > 0):
        if(i == (len(all_albums)/20).__ceil__()):
            all_artists = artistsFromAlbums(get_multiple_albums(all_albums[(i-1)*20:], "US"), all_artists)
        else:
            all_artists = artistsFromAlbums(get_multiple_albums(all_albums[(i-1)*20:i*20], "US"), all_artists)
        i-=1

    return all_artists

# Makes a playlist for a specified artist
# name is the name of the playlist, if their is none then it is the current artist
def makePlaylist(name=None):
    if(name == None):
        name = songInfo()["artists"][0] #Gets first name if there are features

    username = get_current_user_profile()["display_name"]
    playlist = create_playlist(username, name, True, "This is a playlist made by the desktop application RateIt.")

    return playlist["id"]

# Adds songs to an existing playlist
# playlist_id is the id of a playlist, all_songs are the songs that are being added
def addToPlaylist(playlist_id, all_songs):
    i = (len(all_songs)/100).__ceil__() # Can only add a maximum of 100 songs
    while(i > 0):
        if(i == (len(all_songs)/100).__ceil__()):
            print(add_playlist_items(playlist_id, all_songs[(i-1)*100:]))
        else:
            add_playlist_items(playlist_id, all_songs[(i-1)*100:i*100])
        i-=1

# Creates a playlist for multiple artists
# artists is a list of PlaylistPeople
# TO-DO: Test to make sure this works
def makeGroupPlaylist(name:str, artists):
    id = makePlaylist(name)
    all_songs = list()
    for person in artists:
        if type(person.getText()) != int:
            continue

        min_rating = int(person.getText())
        for sid in getRatedSongs(person.getArtist(), min_rating, id):
            all_songs.append(sid)

    for i in range(0, len(all_songs), 100):
        print(add_playlist_items(id, all_songs[i:i+100]))

def main():
    # rateCurSong(9)
    # reRate(8)

    # song = songInfo("21jGcNKet2qwijlDFuPiPb")
    # print(song)
    # print(artistAlbums("7jVv8c5Fj3E9VhNjxT4snq", "uri"))
    # print(getAlbumSongs("4ERnEGlYApJgSQ57j2qlXC", "7jVv8c5Fj3E9VhNjxT4snq", "uri"))
    # playDiscography()
    # findFeatures("Kendrick Lamar", "2YZyLoL8N0Wb9xBt1NhZWg")
    
    pass

if __name__ == "__main__":
    main()