# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_watch_shows_scenario

# Watch Shows Card Scenario - "Normal Viewing"
label card_watch_shows_scenario:
    scene vn-10

    show mi

    mi "Time for my evening ritual - settling in with the TV remote."

    mi "The couch has that perfect indentation where I always sit."

    mi "Remote in hand, snacks within reach, the world ready to fade away."

    mi "There's something comforting about other people's problems playing out on screen."

    mi "Their dramas are contained, resolved in neat episodes or seasons."

    mi "Unlike real life, where nothing ever seems to get tied up nicely."

    mi "Tonight it's a comedy series I've seen three times already."

    mi "But that's okay. Familiar jokes are like old friends."

    mi "The laugh track tells me when to smile, even when I don't feel like it."

    mi "For an hour or two, I can pretend I'm part of their world."

    mi "Where problems are quirky instead of overwhelming."

    mi "Where awkward situations lead to growth instead of isolation."

    mi "The glow from the TV screen paints everything in soft blues and whites."

    mi "It's like being in a bubble of artificial light and sound."

    mi "Safe from the real world outside these walls."

    mi "Maybe this is what peace feels like for someone like me."

    hide mi

    "Mi sinks deeper into the couch cushions, fully absorbed in the screen."

    "Hours pass without notice, episode after episode flowing together."

    "The outside world continues its pace, but here time moves differently."

    "In this cocoon of fictional stories, reality can wait."

    "Sometimes escape isn't running away - it's just taking a break."
