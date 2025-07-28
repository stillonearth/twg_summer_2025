# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_take_shower_scenario

# Take Shower Card Scenario - "Cleansing Ritual"
label card_take_shower_scenario:
    scene vn-7

    show mi

    mi "I should probably take a shower today."

    mi "When was the last time? Three days ago? Four?"

    mi "Time blurs together when you don't leave the house much."

    mi "The bathroom mirror shows someone I barely recognize."

    mi "Hair greasy, skin dull, eyes tired."

    mi "But there's something hopeful about turning on the water."

    mi "The sound of it hitting the tiles is almost musical."

    mi "Steam begins to rise, fogging up the glass."

    mi "Maybe this is what renewal feels like."

    mi "The hot water cascades over my shoulders, washing away more than just dirt."

    mi "It's washing away the lethargy, the stagnation, the feeling of being stuck."

    mi "For these few minutes, I'm present in my own body again."

    mi "The soap smells fresh and clean, like possibilities."

    mi "I take my time, scrubbing away not just the grime but the heaviness."

    mi "Steam surrounds me like a protective cocoon."

    mi "In here, the outside world doesn't exist."

    mi "Just me, the water, and this simple act of taking care of myself."

    hide mi

    "Mi stands under the shower longer than necessary."

    "The warm water creates a meditative rhythm against her skin."

    "When she finally steps out, the mirror is completely fogged."

    "But somehow, she feels clearer than she has in days."

    "Clean towels, clean body, and maybe the beginning of a clean slate."
