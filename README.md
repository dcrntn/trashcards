# *T*e*r*minal fl*ashcards*

The concept of this project is to create a terminal based flashcards software. So it'd be possible for users to learn from flashcards in the terminal. __Please note that there are already existing implementations for such an application (and you should check those out).__ This is a project to test stuff that doesn't make much sense and just to mess around.

# How to use
1. Load a dataset csv in the settings, by pressing key: "2" > "l"
2. Start the game by pressing key: "1" > "s"

- You can quit by pressing "q"

# Current state

The base of the app works. You can load a csv and loop thru the questions/answers. In the csv, the first column is the question and all other columns are the answers, for example if you want to learn multiple languages (prolly not prog. langs) at once, you can do it with this app. You can generate the CSVs by yourself, or just get some AI model to generate it. The spearator should be "__;__".

# Planned features
- ~~Some sort of AI API to fetch questions/answers from the app itself.~~ __(Yeet)__
- Some sort of compression algo, so instead of plain csv, it can be compressed to a .trsh file that should be smaller in size.
- Voice record so you can record yourself to be able to listen to all the questions/answers. (Since learning by listening is prolly more convenient for many)
- P2P connection of some sort, so co-learning could be a thing.

# Yeeted feautres
 - Checked out AI (Gemini) REST API implementation to fetch questions/answers from the app itself. __Dropped because I'm not willing to handle async as of now__

# The END GOAL

![HUH](huhcat.gif)

Is basically this gif, so to have an app that functions, but has questionable implementations in it. 
