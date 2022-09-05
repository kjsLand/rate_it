from tkinter import Button
import home
from playing import rateCurSong

class StarLine():
    __slots__ = ["__head", "__tail"]

    def __init__(self, amount, master):
        cur = None
        prev = None
        for x in range(1, amount+1):
            cur = Star(x, prev, master)
            if(x == 1):
                self.__head = cur
            cur.grid(column=x, row=0)
            prev = cur
        self.__tail = prev

class Star(Button):
    __slots__ = ["__positon" , "__prev"]

    def __init__(self, position, prev=None, *args, **kwargs):
        self.__position = position
        self.__prev = prev
        Button.__init__(self, *args, **kwargs)
        self.img = home.create_image("./media/star.png", 30, 30)
        self.config(image=self.img)
        self["border"] = False
        self["bg"] = "#C0C0C0"
        self["command"] = lambda: rateCurSong(self.__position)
        self.bind("<Enter>", lambda y: self.light_change(self, "./media/full_star.png"))
        self.bind("<Leave>", lambda y: self.light_change(self, "./media/star.png"))

    def get_prev(self):
        return self.__prev

    def set_prev(self, prev):
        self.__prev = prev
    
    def get_positon(self):
        return self.__position

    def light_change(self, cur, filename):
        if(cur is None):
            pass
        else:
            cur.img = home.create_image(filename, 30, 30)
            cur.configure(image=cur.img)
            cur.light_change(cur.get_prev(), filename)