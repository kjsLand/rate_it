from PIL import ImageTk, Image

def create_image(filepath, height=50, width=50):
    img = Image.open(filepath) #opens image
    img = img.resize((width, height)) #resizes image
    img = ImageTk.PhotoImage(img)# PhotoImage class is used to add image to widgets, icons etc
    return img