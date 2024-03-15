from tkinter import *
from tkinter import ttk
import requests
from HelperFrames import FeaturedPeople, PlaylistPeople, artistRatingVeiw, discoverSection
from editor import getAllArtists, getOverallRating, getRatedSongs, getTopRated, isRated
from playing import artistAlbums, findFeatures, makeGroupPlaylist, reRate, songInfo, playAlbum, playDiscography, playTopTen
from spotify_functions.albums import get_album_tracks
from spotify_functions.artists import get_artist
from spotify_functions.player import add_to_queue
from PIL import ImageTk, Image

# Global Variables
URL_PHOTO_SIZE = 200

def urlPhoto(root, imageUrl):
    # https://stackoverflow.com/questions/7391945/how-do-i-read-image-data-from-a-url-in-python
    # --------- Credit for line below -----------
    img = Image.open(requests.get(imageUrl, stream=True).raw)
    img = img.resize((URL_PHOTO_SIZE, URL_PHOTO_SIZE)) # This needs to be a tuple
    photo = ImageTk.PhotoImage(img)
    label = Label(root, image=photo)
    label.image = photo
    return label

# This is repeated - fix it later
def create_image(filepath, height=50, width=50):
    img = Image.open(filepath) #opens image
    img = img.resize((width, height)) #resizes image
    img = ImageTk.PhotoImage(img)# PhotoImage class is used to add image to widgets, icons etc
    return img

# This function makes the scroll frame for the Related and Playlist page
# --------------- Credit to codemy.com for the creation of the scroll bar --------------
def create_scroll(root):
    # Creates a canvas
        canvas = Canvas(root, width=350, bg="#D0E8FF")
        canvas.pack(side=RIGHT, fill=BOTH)

        # Add a Scrollbar
        scroll = ttk.Scrollbar(root, orient=VERTICAL, command=canvas.yview)
        scroll.pack(side=RIGHT, fill=Y)

        # Configure the Canvas
        canvas.configure(yscrollcommand=scroll.set)
        canvas.bind("<Configure>", lambda e:canvas.configure(scrollregion=canvas.bbox("all")))

        # Frame inside Canvas
        frame = Frame(canvas, bg="#D0E8FF")
        canvas.create_window((0,0), window=frame, anchor="nw")

        return frame

def text_from_feild(string):
    return string[:len(string)-1]

# This is the parent class of all pages that extend off of the home page.
# The shared features are a back button(brings user back to home) and a get page function
# Only altered method for child clases is the make function
class Page():
    __slots__ = ["__page"]

    def __init__(self, root, home_page):
        self.__page = Frame(root, bg="#C0C0C0")

        # Back button
        backImg = create_image("./media/back.png", 45, 65)
        back = Button(self.__page, bg="#C0C0C0", border=False, image=backImg, command=lambda:home_page.tkraise())
        back.place(relx=0.05, rely=0.05, anchor=CENTER)
        back.image = backImg

        # Makes the contents of the page
        self.make()

    def clear(self, frame):
        for widget in frame.winfo_children():
            widget.destroy()

    # Returns the frame of the page
    def getPage(self):
        return self.__page
        
    # Makes a button to navigate to said page
    def navButton(self, frame, button_img):
        return Button(frame, border=False, bg="#C0C0C0", image=button_img, command=lambda:self.getPage().tkraise())

    # Creates the contents of the page
    def make(self):
        pass

# Discover class is a child of Page
# Its function is to create a page for the Discover button
class Discover(Page):
    def __init__(self, root, home_page):
        super().__init__(root, home_page)

    def make(self):
        main = Frame(self.getPage(), bg="#C0C0C0")
        main.place(relx=0.5, rely=0.5, anchor=CENTER)

        # Play full discography section
        fullText = "\n This button creates a playlist full of songs from the current artist's discography." \
        " Find the playlist in your library under the artist's name.\n"
        discoverSection(main, "#D0E8FF", "#75BAFF", "Full Discography", fullText, playDiscography, 0)

        # Play all unrated songs
        unratedText = "\nThis button creates a playlist full of all unrated songs from the current artist's" \
        "discography. Find the playlist in your library under the artist's name with (unrated) after."
        discoverSection(main, "#FFFF9F", "#FFC90E", "Unrated Songs", unratedText, playDiscography, 1)

        # Play all of album
        albumText = "\nThis button adds all songs from the current album to the queue.\n\n\n"
        discoverSection(main, "#D0E8FF", "#75BAFF", "Album", albumText, playAlbum, 2)

        # Play top 10 songs
        topTenText = "\nThis button adds the top 10 songs of the current artist to the queue.\n\n\n"
        discoverSection(main, "#FFFF9F", "#FFC90E", "Top 10", topTenText, playTopTen, 3)

# Share class is a child of Page
# Its function is to create a page for the Share button
class Share(Page):
    def __init__(self, root, home_page):
        super().__init__(root, home_page)

    def make(self):
        frame = Frame(self.getPage(), bg="#75BAFF", borderwidth="2", relief="groove")

        song = Button(self.getPage(), text="Check Current Song Rating", bg="#D0E8FF", command=lambda:self.songRating(frame))
        song.place(relx=0.25, rely=0.05, anchor=CENTER)

        album = Button(self.getPage(), text="Check Current Album Rating", bg="#D0E8FF", command=lambda:self.albumRating(frame))
        album.place(relx=0.5, rely=0.05, anchor=CENTER)

        clear = Button(self.getPage(), text="Clear", bg="#D0E8FF", command=lambda:self.clear(frame))
        clear.place(relx=0.75, rely=0.05, anchor=CENTER)

        labelText = "Snap a picture of the rating and send it to a friend."
        label = Label(self.getPage(), text=labelText, font=('Helvetica', 10, 'bold'), bg="#C0C0C0")
        label.place(relx=0.5, rely=0.95, anchor=CENTER)

        frame.place(relx=0.5, rely=0.5, anchor=CENTER)
    
    def songRating(self, frame):
        info = songInfo()
        rating = isRated(info["song id"], info["artists"][0])

        if(rating == False):
            notRated = Label(frame, text="This song is not rated yet.", bg="#D0E8FF", font=('Helvetica', 15, 'bold'))
            notRated.pack()
        else:
            name = Label(frame, bg="#75BAFF", text=info["song name"], font=('Helvetica', 15, 'bold'))
            name.grid(row=0, column=0)

            albumCover = urlPhoto(frame, info["imgurl"])
            albumCover.grid(row=1, column=0)

            rated = Label(frame, bg="#75BAFF", text="Rating: " + rating[1], font=('Helvetica', 15, 'bold'))
            rated.grid(row=2, column=0)

    def albumRating(self, frame):
        info = songInfo()
        album = get_album_tracks(info["album id"])

        # Make frames
        albumInfo = Frame(frame, bg="#75BAFF")
        albumInfo.grid(row=0, column=0)
        trackInfo = Frame(frame,  bg="#75BAFF")
        trackInfo.grid(row=0, column=1)

        # Fill in track info
        songs = dict()
        for track in album["items"]:
            songs[track["name"]] = track["id"]

        ratings = list()
        total = 0
        ratedTotal = 0
        for key in songs:
            cur_song = isRated(songs[key], info["artists"][0])
            
            if(cur_song == False):
                ratings.append(key)
            else:
                ratings.append(cur_song)
                ratedTotal+=1
                total += (int)(cur_song[1])

        i = 0
        for rate in ratings:
            if(type(rate) == type("")):
                Label(trackInfo, bg="#75BAFF", text=rate).grid(row=i, column=0)
                Label(trackInfo, bg="#75BAFF", text="Not Rated").grid(row=i, column=1)
            else:
                Label(trackInfo, bg="#75BAFF", text=rate[0]).grid(row=i, column=0)
                Label(trackInfo, bg="#75BAFF", text="Rated: " + rate[1]).grid(row=i, column=1)
            i+=1

        # Fill in album info
        albumName = Label(albumInfo, text=info["album name"], bg="#75BAFF")
        albumName.grid(row=0, column=0)

        albumCover = urlPhoto(albumInfo, info["imgurl"])
        albumCover.grid(row=1, column=0)

        if ratedTotal == 0:
            overall = 0
        else:
            overall = (str)((total/ratedTotal).__round__(1))
        avgRating = Label(albumInfo, text="Overall Rating: " + overall, bg="#75BAFF")
        avgRating.grid(row=2, column=0)

        albumInfo.configure(height=trackInfo.winfo_height()) # Makes both frames the same height

# Playlist class is a child of Page
# Its function is to create a page for the Playlist button
class Playlist(Page):
    def __init__(self, root, home_page):
        super().__init__(root, home_page)

    def make(self):
        frame = create_scroll(self.getPage())

        i = 0
        artists = list()
        for name in getAllArtists():
            person = PlaylistPeople(frame, name)
            artists.append(person)
            person.getFrame().grid(row=i, column=0)
            i+=1

        playlistName = Text(self.getPage(), height=1, width=50)
        playlistName.place(relx=0.40, rely=0.45, anchor=CENTER)
        Button(self.getPage(), bg="#D0E8FF", text="Make Playlist", command=lambda:makeGroupPlaylist(playlistName.get("1.0", END), artists)).place(relx=0.20, rely=0.45, anchor=CENTER)
        instructText = "This page helps to create playlists from your past ratings. " + \
        "Enter a number 1-9 for any amount of the artists on the right. Then type in a name and " +\
        "click the button to make a playlist. "
        instructions = Label(self.getPage(), text=instructText, wraplength=400, bg="#C0C0C0", font=('Helvetica', 12, 'bold'))
        instructions.place(relx=0.35, rely=0.65, anchor=CENTER)

# Related class is a child of Page
# Its function is to create a page for the Related Artist button
class Related(Page):
    def __init__(self, root, home_page):
        super().__init__(root, home_page)

    def make(self):
        frame = create_scroll(self.getPage())

        tracks = Button(self.getPage(), bg="#D0E8FF", text="Get Features", command=lambda:self.fillData(frame))
        tracks.place(relx=0.30, rely=0.50, anchor=CENTER)

        clear = Button(self.getPage(), bg="#D0E8FF", text="Clear", command=lambda:self.clear(frame))
        clear.place(relx=0.40, rely=0.50, anchor=CENTER)

        instructText = "This page helps to find new artists through features on songs. " +\
        "Click the button to see all the people that the artists you are listening to has featured with." +\
        " If the scroll bar is not working, try minimizing your window and then go back to full screen."
        instructions = Label(self.getPage(), text=instructText, wraplength=400, bg="#C0C0C0", font=('Helvetica', 12, 'bold'))
        instructions.place(relx=0.35, rely=0.65, anchor=CENTER)
    
    def fillData(self, root):
        featured = findFeatures()
        i = 0
        for key in featured:
            featPeople = FeaturedPeople(root, key, featured[key])
            featPeople.getFrame().grid(row=i, column=0)
            i+=1

# Current class is a child of Page
# Its function is to create a page for the Current button
class Current(Page):
    def __init__(self, root, home_page):
        super().__init__(root, home_page)

    def make(self):
        content = Frame(self.getPage(), bg="#C0C0C0")

        get_artist = Button(self.getPage(), text="Get Artist", bg="#D0E8FF", command=lambda:self.updateInfo(content))
        get_artist.place(relx=0.40, rely=0.05, anchor=CENTER)

        clear = Button(self.getPage(), text="clear", bg="#D0E8FF", command=lambda:self.clear(content))
        clear.place(relx=0.60, rely=0.05, anchor=CENTER)

        content.place(relx=0.5, rely=0.5, anchor=CENTER)
    
    def updateInfo(self, root):
        info = songInfo()
        artist = info["artists"][0]
        artistInfo = get_artist(info["artists id"][0])

        name = Label(root, text=artist, font=('Helvetica', 25, 'bold'), bg="#C0C0C0")
        name.grid(row=0, column=0)

        rating = Label(root, text="rating: " + (str)(getOverallRating(artist)), font=('Helvetica', 25, 'bold'), bg="#C0C0C0")
        rating.grid(row=1, column=0)

        albumCover = urlPhoto(root, artistInfo["images"][0]["url"])
        albumCover.grid(row=2, column=0)

        discography_info, total_songs = artistAlbums(info["artists id"][0], "id")
        rated_fraction = (str)(len(getRatedSongs(artist, 0))) + "/" + (str)(total_songs)
        progress = Label(root, text=rated_fraction, font=('Helvetica', 25, 'bold'), bg="#C0C0C0")
        progress.grid(row=3, column=0, pady=(0,18))

        i = 0
        for song in getTopRated(artist):
            frame = Frame(root, bg="#75BAFF", borderwidth="2", relief="groove")
            Label(frame, text=song[0], wraplength=100, width=50, bg="#75BAFF").grid(row=0, column=0, padx=(0, 30))
            Label(frame, text="Rating: "+song[1], bg="#75BAFF").grid(row=0, column=3, padx=(0, 30))
            Button(frame, text="Play", bg="#D0E8FF", command=lambda:add_to_queue(song[2])).grid(row=0, column=4)
            frame.grid(row=4+i, column=0)
            i+=1

# Ratings class is a child of Page
# Its function is to create a page for the Ratings button
class Ratings(Page):
    def __init__(self, root, home_page):
        super().__init__(root, home_page)
    
    def make(self):
        scroll = create_scroll(self.getPage())

        Button(self.getPage(), text="Show Artists", bg="#75BAFF", command=lambda:self.showArtists(scroll)).place(relx=0.3, rely=0.5, anchor=CENTER)
        rating = Text(self.getPage(), height=1, width=2)
        rating.place(relx=0.45, rely=0.5, anchor=CENTER)
        Button(self.getPage(), text="rerate", bg="#75BAFF", command=lambda:reRate((int)(text_from_feild(rating.get("1.0", END))))).place(relx=0.4, rely=0.5, anchor=CENTER)
    
    def showArtists(self, frame):
        self.clear(frame)
        index = 0
        artists = getAllArtists()

        for person in artists:
            button = artistRatingVeiw(person, frame, frame)
            button.grid(row=index, column=0, padx=(25, 0))
            index += 1