# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_3am_thoughts_scenario

# 3AM Thoughts Card Scenario - "3AM Existential Crisis"
label card_3am_thoughts_scenario:
    scene vn-23

    show mi

    mi "3:17 AM. The time when all the big questions come crawling out of the dark."

    mi "What am I doing with my life? What's the point of any of this?"

    mi "The ceiling has become my philosopher's canvas tonight."

    mi "Everyone else is sleeping peacefully while I'm here questioning the nature of existence."

    mi "Do my actions matter? Does anyone's?"

    mi "We're all just tiny specks on a rock floating through infinite space."

    mi "And somehow I'm supposed to care about paying bills and maintaining relationships?"

    mi "The darkness makes everything feel more intense, more real."

    mi "During the day, I can distract myself with screens and noise."

    mi "But at 3 AM, there's nowhere to hide from the big questions."

    mi "Am I wasting my life? Is there even such a thing as wasting it?"

    mi "Maybe everyone is just pretending they know what they're doing."

    mi "Maybe we're all equally lost, just better at hiding it."

    mi "The clock ticks away seconds that I'll never get back."

    mi "Each tick a reminder that time is finite and I'm spending it lying here paralyzed."

    mi "But what if paralysis is the most honest response to an absurd world?"

    mi "3:47 AM. Still no answers, just more questions."

    hide mi

    "Mi stares into the darkness, overwhelmed by the vastness of everything."

    "The night stretches endlessly, filled with thoughts too big for any one mind."

    "Sleep feels impossible when your brain is trying to solve the universe."

    "These are the hours when existence feels both precious and pointless."

    "Eventually exhaustion will win, but tonight the questions reign supreme."
