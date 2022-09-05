LOCAL_DATABASE = "./rate.txt"

# Credit https://www.codegrepper.com/code-examples/python/python+write+to+middle+of+file 
# Took inspiration from the link above
def write(words, index):
    with open(LOCAL_DATABASE) as file:
        contents = file.readlines() # Same thing as iterating over all line and appending them

    contents.insert(index, words) 

    with open(LOCAL_DATABASE, "w") as file:
        contents = "".join(contents) # Turns list to a string
        file.write(contents) # Writes contents to the file

def writeEndOfLine(words, index):
    with open(LOCAL_DATABASE) as file:
        contents = file.readlines() # Same thing as iterating over all line and appending them

    temp = contents[index][:len(contents[index])-1]
    contents.pop(index)
    temp += words + "\n"
    contents.insert(index, temp)

    with open(LOCAL_DATABASE, "w") as file:
        contents = "".join(contents) # Turns list to a string
        file.write(contents) # Writes contents to the file

def findPlacement(name):
    line_num = 0
    with open(LOCAL_DATABASE) as file:
        for line in file:
            tokens = line.split(":")
            line_artist = tokens[0]
            try:
                for index in range(len(line_artist)):
                    if ord(line_artist[index].lower()) == ord(name[index].lower()):
                        continue # if letters are the same
                    elif ord(line_artist[index]) > ord(name[index]):
                        return line_num # if letter of given name is bigger
                    else:
                        line_num += 1
                        break # if letter of given name is smaller
            except IndexError:
                return line_num # if index is too big for either name
        return line_num # if the name already went through all names in file

def addArtist(name):
    write(name + ":\n", findPlacement(name))

def findArtistLine(artist):
    with open(LOCAL_DATABASE) as file:
        i = 0
        for line in file:
            tokens = line.split(":") #Splits by the semicolon
            name = tokens[0][:len(tokens[0])] #Gets rid of semicolon at the end of name in rate.txt
            if(artist == name):
                return i
            else:
                i += 1
        return -1

# Check Characters is a helper functon for addSong
# Makes sure that no problematic characters are in data that is stored
# commas, colons, and pipe have to be removed because they are seperators for the data
# foreign characters have to be replaced, if they aren't then a Unicode error is thrown and 
# deletes all the data that a user has
# String parameter is an artist's name or song title
def checkCharacters(string):
    end_of_string = len(string)
    i = 0

    while(i < end_of_string):
        if string[i] == "," or string[i] == ":" or string[i] == "|": # These are the characters that split the data
            string = string[:i] + string[i+1:]
            end_of_string-=1
        elif len(ascii(string[i])) > 3: # Handles foreign characters that can't be stored in a text file
            string = string[:i] + "?" + string[i+1:]
        i+=1

    return string

def addSong(name, rating, song_id, artist):
    index = findArtistLine(artist)

    # Handles problematic characters in the song's and artist's name
    name = checkCharacters(name)
    artist = checkCharacters(artist)

    # Extra comma at the end is used to prevent an error
    # Without the comma, the new line character will break later functions when reading the file
    if(index > -1):
        writeEndOfLine(f"|{name},{rating},{song_id},", index) 
    elif(index == -1):
        addArtist(artist)
        writeEndOfLine(f"|{name},{rating},{song_id},", findArtistLine(artist))

# Removes a song from the local database
def removeSong(song_id, artist):
    artistIndex = findArtistLine(artist)
    
    with open(LOCAL_DATABASE) as file:
        contents = file.readlines() # Same thing as iterating over all line and appending them

    temp = contents[artistIndex][:len(contents[artistIndex])-1] # Gets rid of newline character
    contents.pop(artistIndex)
    afterName = temp.split(":")
    afterName.pop(0)
    all_songs = afterName[0].split("|")
    
    cur_song = isRated(song_id, artist)

    if(len(cur_song) == 4): # Removes newline character if it is the last song
        cur_song[len(cur_song)-1] = ""
    if(cur_song == False): # Stops the function if the song was not previously rated
        return False
    
    all_songs.remove(",".join(cur_song))

    temp = artist + ":" + "|".join(all_songs) + "\n"
    contents.insert(artistIndex, temp)

    with open(LOCAL_DATABASE, "w") as file:
        contents = "".join(contents) # Turns list to a string
        file.write(contents) # Writes contents to the file

# Returns the song if rated, false otherwise
def isRated(song_id, artist_name):
    index = findArtistLine(artist_name)
    with open(LOCAL_DATABASE) as file:
        contents = file.readlines()
        afterName = contents[index].split(":")[1]
        songs = afterName.split("|")
        songs.pop(0)
        for element in songs:
            field = element.split(",")
            if(field[2] == song_id):
                return field
    return False

def getAllArtists():
    all_names = list()
    with open(LOCAL_DATABASE) as file:
        for line in file:
            tokens = line.split(":") #Splits by the semicolon
            name = tokens[0][:len(tokens[0])] #Gets rid of semicolon at the end of name in rate.txt
            all_names.append(name)
    return all_names

# type options: rating, name, id, none gives all info
def getRatedSongs(artist_name, rating, type=None):
    tracks = list()
    index = findArtistLine(artist_name)

    if(index == -1): # When an artist isn't fount
        return tracks

    with open(LOCAL_DATABASE) as file:
        contents = file.readlines()
        afterName = contents[index].split(":")[1]
        songs = afterName.split("|")
        songs.pop(0) # Gets rid of first bar
        for item in songs:
            song_info = item.split(",")
            if((int)(song_info[1]) >= rating):
                match type:
                    case "name":
                        tracks.append(song_info[0])
                    case "rating":
                        tracks.append(song_info[1])
                    case "id":
                        tracks.append(song_info[2])
                    case default:
                        tracks.append(song_info[:3])
    return tracks

def getOverallRating(artist_name):
    all_songs = getRatedSongs(artist_name, 0, "rating")
    total = 0
    for num in all_songs:
        total += (int)(num)

    try: 
        return (total/len(all_songs)).__round__(1)
    except ZeroDivisionError:
        return 0

def getTopRated(artist_name):
    all_songs = getRatedSongs(artist_name, 0)
    all_songs.sort(key=lambda x: x[1], reverse=True)
    return all_songs[:5]

def main():
    # print(getAllArtists())
    # print(getRatedSongs("Joji", 5))
    # print(getOverallRating("Joji"))
    # print(getTopRated("Post Malone"))
    # print(isRated("7FCHDXa579YivyHrsxixRp", "iann dior"))

    # while(True):
    #     name = input("Enter an artists name: ")
    #     if(name == ""):
    #         break
    #     addArtist(name)

    # print(findArtistLine("Kendrick Lamar"))

    # addSong("Go Legend", 3, "rap", "Big Sean")
    # addSong("6 Man", 5, "rap", "Drake")

    pass

if __name__ == "__main__":
    main()