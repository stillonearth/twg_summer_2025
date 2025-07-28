# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_binge_series_scenario

# Binge Series Card Scenario - "Marathon Session"
label card_binge_series_scenario:
    scene vn-11

    show mi

    mi "This is it. Today I'm going to finish the entire season."

    mi "I've got snacks, drinks, and absolutely no plans to move from this spot."

    mi "The remote is locked and loaded with 'next episode' queued up."

    mi "Who needs sunlight when you have the glow of a 55-inch screen?"

    mi "Episode one... I remember when I thought I'd just watch a couple."

    mi "That was eight hours ago."

    mi "Now there are empty containers scattered around like evidence of my commitment."

    mi "The characters on screen have become more real to me than actual people."

    mi "I know their names, their relationships, their deepest secrets."

    mi "They're better company than most humans anyway."

    mi "The blanket has become part of me now, a cozy armor against the world."

    mi "Time has lost all meaning. Is it morning? Evening? Tuesday?"

    mi "The only clock that matters is the episode runtime counter."

    mi "My eyes are getting dry, but I can't look away."

    mi "What if something important happens in the next episode?"

    mi "What if there's a cliffhanger and I have to wait?"

    mi "No, better to push through and reach the conclusion."

    hide mi

    "Mi reaches for another snack without taking her eyes off the screen."

    "The outside world might as well not exist during these marathon sessions."

    "Hours dissolve into a blur of plot twists and character development."

    "By the time the season finale ends, it's a different day entirely."

    "But somehow, she feels like she's lived through multiple lifetimes in this one spot."
