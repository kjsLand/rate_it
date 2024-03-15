import os
import signal
import threading
import webbrowser
from .basics import TOKEN
from appCredentials import PASSWORD, USERNAME
from fastapi import FastAPI
from fastapi.responses import HTMLResponse
import requests
import uvicorn

app = FastAPI()
client_id = USERNAME
client_secret = PASSWORD
redirect_uri = 'http://127.0.0.1:8080/callback/'
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

# Stops the local server when called
def close_server():
    os.kill(os.getpid(), signal.SIGTERM)

def auth_setup():
    local_server = threading.Thread(target=uvicorn.run, args=(app,), kwargs={'host':"127.0.0.1", 'port':8080})
    local_server.start()
    webbrowser.open_new_tab(f"https://accounts.spotify.com/authorize?response_type=code&client_id={client_id}&redirect_uri={redirect_uri}&scope={' '.join(scope)}")

def get_access_token(auth_code: str):
    response = requests.post(
        "https://accounts.spotify.com/api/token",
        data={
            "grant_type": "authorization_code",
            "code": auth_code,
            "redirect_uri": redirect_uri,
        },
        auth=(client_id, client_secret),
    )
    TOKEN.set_tokens(response.json())

@app.get("/")
async def auth():
    return HTMLResponse(content=f'<p>root</p>')

@app.get("/callback/")
async def callback(code:str):
    try:
        get_access_token(code)
        return HTMLResponse(content=f'<p>You are now authorized!</p>')
    except KeyError:
        return HTMLResponse(content=f'<p>Authorization Failed - code entered was invalid</p>')