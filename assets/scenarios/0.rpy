# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_sleep_scenario

# Sleep Card Scenario - "Peaceful Night"
label card_sleep_scenario:
    # Set up the scene
    scene vn-0

    # Character enters scene
    show mi

    mi "Another long day of... well, not much really."

    mi "The same routine. Wake up late, browse the internet, eat convenience store food, and somehow the day just disappears."

    mi "But now... the bed looks so inviting."

    mi "The moonlight through the window... it's actually quite peaceful."

    mi "Sometimes I wonder what dreams will come tonight."

    mi "Will they be about the outside world I've been avoiding?"

    mi "Or just more of the same endless digital landscapes I escape into?"

    mi "The night sky... when was the last time I really looked at it?"

    mi "There are people out there, living their lives, having adventures..."

    mi "And here I am, watching from behind this window like always."

    mi "But you know what? Tonight, that's okay."

    mi "Sometimes the observer sees things others miss."

    mi "The way the moonlight catches the edges of clouds..."

    mi "The distant sound of a train carrying people to unknown destinations..."

    mi "Maybe tomorrow will be different. Or maybe it won't."

    mi "But tonight, I'll dream."

    hide mi

    "Mi slowly prepares for bed, mind still turning over these quiet thoughts."

    "Within minutes, the gentle rhythm of breathing fills the quiet room."

    "As Mi drifts into sleep, the boundaries between reality and dreams begin to blur..."

    "In dreams, even a hikikomori can be anywhere, anyone..."

    "Mi sleeps peacefully through the night, gathering energy for whatever tomorrow might bring."
