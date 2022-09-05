from tkinter import END, Button, Frame, Label, Text
from editor import getRatedSongs
from playing import playTopTen

# Showcases an artist from the local database
class PlaylistPeople:
    __slots__ = ["__artist", "__frame", "__text_field"]

    def __init__(self, root, artist):
        self.__artist = artist
        self.__frame = Frame(root, bg="#D0E8FF", pady=5)
        label = Label(self.__frame, text=self.__artist, bg="#D0E8FF", width=10, wraplength=80, font=('Helvetica', 10))
        label.grid(row=0, column=1)
        self.__text_field = Text(self.__frame, height=1, width=2)
        self.__text_field.grid(row=0, column=0, padx=(60,0))
    
    def getFrame(self):
        return self.__frame

    def getText(self):
        return self.__text_field.get("1.0", END)
    
    def getArtist(self):
        return self.__artist

# Showcases an artist from the local database
class FeaturedPeople:
    __slots__ = ["__frame"]

    def __init__(self, root, artist, artist_id):
        self.__frame = Frame(root, bg="#D0E8FF", pady=5)
        label = Label(self.__frame, text=artist, width=10, wraplength=80, bg="#D0E8FF")
        label.grid(row=0, column=0, padx=(85, 10))
        button = Button(self.__frame, bg="#75BAFF", text="Play Top Ten", command=lambda:playTopTen(artist_id))
        button.grid(row=0, column=1)
    
    def getFrame(self):
        return self.__frame

# Creates a frame for the discovery page
class discoverSection:
    __slots__ = ["__frame"]

    def __init__(self, root, labelColor, buttonColor, buttonTitle, txt, func, ord):
        frame = Frame(root, bg=labelColor, padx=35, pady=35, borderwidth="2", relief="groove")
        frame.grid(row=0, column=ord)

        if(buttonTitle == "Unrated Songs"):
            button = Button(frame, bg=buttonColor, text=buttonTitle, command=lambda:func(unrated_mode=True))
        else:
            button = Button(frame, bg=buttonColor, text=buttonTitle, command=lambda:func())
        button.grid(row=0, column=0)

        label = Label(frame, text=txt, wraplength=200, bg=labelColor, font=('Helvetica', 10, 'bold'))
        label.grid(row=1, column=0)

    def getFrame(self):
        return self.__frame

# creates a list of ratings from that artist for the user to veiw.
class artistRatingVeiw(Button):
    __slots__ = ["__artist", "__frame"]

    def __init__(self, artist, frame, *args, **kwargs):
        Button.__init__(self, *args, **kwargs)
        self.__artist = artist
        self.__frame = frame
        self["text"] = artist
        self["command"] = lambda:self.displayRating()

    def displayRating(self):
        for widget in self.__frame.winfo_children():
            widget.destroy()

        Label(self.__frame, text=self.__artist, bg="#75BAFF").grid(row=0, column=0)

        index = 1
        for track in getRatedSongs(self.__artist, 0):
            song = Frame(self.__frame, bg="#75BAFF")
            song.grid(row=index, column=0)
            index += 1

            i = 1
            for item in track:
                if(i!=3):
                    Label(song, text=item, bg="#D0E8FF").grid(row=0, column=i)
                i+=1
    