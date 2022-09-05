####### Credit to the below link for the authorization process ######
####### https://github.com/requests/requests-oauthlib/blob/master/docs/examples/spotify.rst ########

from tkinter import CENTER, Button, Label, Text
import webbrowser
from requests_oauthlib import OAuth2Session
from requests.auth import HTTPBasicAuth
from .basics import TOKEN
from appCredentials import PASSWORD, USERNAME

def auth_setup(frame):
    # Clears frame
    for widget in frame.winfo_children():
            widget.destroy()

    # Credentials you get from registering a new application
    client_id = USERNAME
    client_secret = PASSWORD
    redirect_uri = 'https://localhost:8080/callback/'

    # OAuth endpoints given in the Spotify API documentation
    # https://developer.spotify.com/documentation/general/guides/authorization/code-flow/
    authorization_base_url = "https://accounts.spotify.com/authorize"
    token_url = "https://accounts.spotify.com/api/token"
    # https://developer.spotify.com/documentation/general/guides/authorization/scopes/
    scope = [
        "user-read-playback-state",
        "user-modify-playback-state",
        "user-read-currently-playing",
        "user-read-recently-played", 
        "user-read-private",
        "user-read-email",
        "playlist-modify-public",
        "playlist-modify-private"
    ]

    spotify = OAuth2Session(client_id, scope=scope, redirect_uri=redirect_uri)

    # Redirect user to Spotify for authorization
    authorization_url, state = spotify.authorization_url(authorization_base_url)
    destination = Label(frame, text='Please go here')
    destination.bind("<Button-1>", lambda e:webbrowser.open_new_tab(authorization_url))
    destination.grid(row=0, column=0)

    # Get the authorization verifier code from the callback url
    Label(frame, text="Paste the full redirect URL here: ").grid(row=1, column=0)
    redirect_response = Text(frame, height=1, width=25)
    redirect_response.grid(row=1, column=1)
    auth = HTTPBasicAuth(client_id, client_secret)

    # Fetch the access token
    fetch_token_button = Button(frame, text="Get Authorization Token", command=lambda:TOKEN.set_token(spotify, token_url, redirect_response, auth, frame, auth_setup))
    fetch_token_button.grid(row=1, column=2)
    
    frame.place(relx=0.5, rely=0.6, anchor=CENTER)