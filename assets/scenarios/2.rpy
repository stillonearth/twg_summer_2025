# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Character Image
image mi = "mi.png"

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_rest_in_bed_scenario

# Rest in Bed Card Scenario - "Restless Rest"
label card_rest_in_bed_scenario:
    scene vn-2

    show mi

    mi "Here I am again, lying in bed but completely unable to sleep."

    mi "My body is tired, but my mind... my mind just won't shut off."

    mi "The ceiling looks the same as always, but tonight the shadows seem different."

    mi "Why do I keep thinking about that conversation from three years ago?"

    mi "And that embarrassing thing I did in middle school that nobody probably even remembers."

    mi "The clock is ticking so loudly. When did it become so noticeable?"

    mi "I should probably get up and do something productive, but..."

    mi "What's the point? Tomorrow will just be another day of the same routine."

    mi "Maybe I should check my phone. No, that'll just make it worse."

    mi "The blue light will keep me awake even longer."

    mi "But lying here with these thoughts spinning around and around..."

    mi "Sometimes I wonder if this is what everyone else goes through."

    mi "Do they lie awake thinking about all the things they should have done differently?"

    mi "Or is it just me, trapped in this endless loop of overthinking?"

    mi "The pillow is getting warm. Maybe if I flip it over..."

    hide mi

    "Mi shifts restlessly, trying to find a comfortable position."

    "The rumpled sheets twist around her as she turns from side to side."

    "Outside, the world sleeps peacefully, but in this dim bedroom, rest remains elusive."

    "Minutes feel like hours when your mind refuses to quiet down."

    "Eventually, exhaustion wins over anxiety, and Mi drifts into an uneasy sleep."
