from tkinter import *
from NavPages import Current, Discover, Page, Ratings, Playlist, Related, Share
from RatingStar import StarLine
from spotify_functions.authorization import auth_setup, close_server
from spotify_functions.basics import TOKEN
from playing import LOGS
from image import create_image

# Color Scheme
# Grey : C0C0C0
# Blue : 75BAFF
# Light Blue : #D0E8FF
# Dark Yellow : #FFC90E
# Light Yellow : #FFFF9F

def main():
    # Makes window
    root = Tk()

    # root.iconbitmap("./prototype/app_cover.ico") #set icon (causes an error when using multiple pages)
    root.title("Rate It")
    root.state("zoomed") #setting tkinter window size
    root.rowconfigure(0, weight=1) #creates row of a grid
    root.columnconfigure(0, weight=1) #creates column of a grid

    #Pages of the program
    home_page = Frame(root, bg="#C0C0C0")
    page_list:list[Page] = [home_page, 
                Share(root, home_page), 
                Playlist(root, home_page), 
                Related(root, home_page), 
                Discover(root, home_page), 
                Current(root, home_page),
                Ratings(root, home_page)]

    #Adds those pages to the root grid
    for page in page_list:
        try:
            page.grid(row=0, column=0, sticky="nsew")
        except AttributeError:
            page.getPage().grid(row=0, column=0, sticky="nsew")

    #Shows home page to the user when application is opened
    home_page.tkraise()

    # Label for console output
    output = Label(home_page, text="Rate It", bg="#C0C0C0", font=('Helvetica', 25, 'bold'))
    output.place(relx=0.5, rely=0.35, anchor=CENTER)

    # Creates the 10 stars
    starFrame = Frame(home_page, background="red")
    starFrame.place(relx=0.50, rely=0.45, anchor=CENTER)
    StarLine(10, starFrame)

    # Navigation images/names
    nav_images = [create_image("./media/share.png", 100, 100), create_image("./media/playlist.png", 100, 100), 
    create_image("./media/related.png", 100, 100), create_image("./media/discover.png", 100, 100), 
    create_image("./media/artist.png", 100, 100), create_image("./media/rate.png", 100, 100)]
    nav_titles = ["Share", "Playlist", "Related Artists", "Discover", "Current Artist", "Ratings"]

    # Navigation buttons
    for i in range(2):
        for j in range(3):
            cur_index = (i*3)+j

            # Container to hold button and title
            frame = Frame(home_page, bg="#C0C0C0")
            frame.place(relx=0.15+0.7*i, rely=0.2+j*0.3, anchor=CENTER)

            # Makes nav button
            page_list[cur_index+1].navButton(frame, nav_images[cur_index]).pack()


            # Makes nav title
            Label(frame, text=nav_titles[cur_index], font=('Helvetica', 18, 'bold'), bg="#C0C0C0").pack()

    # Access Token Frame
    access = Frame(home_page, bg="#C0C0C0")
    access.place(relx=0.5, rely=0.2, anchor=CENTER)
    auth_button = Button(access, text="Click Here to Link Account", command=lambda:auth_setup())
    auth_button.grid(row=0, column=0)

    # Logs
    log = Frame(home_page, bg="#222222")
    log.place(relx=0.5, rely=0.75, anchor=CENTER)
    for i in range(10):
        LOGS[i] = Label(log, text="", width=75)
        LOGS[i].grid(row=i, column=0)

    root.protocol("WM_DELETE_WINDOW", close_server)
    root.mainloop()

if(__name__ == "__main__"):
    main()