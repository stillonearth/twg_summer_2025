# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Character Image
image mi = "mi.png"

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_quick_nap_scenario

# Quick Nap Card Scenario - "Afternoon Rest"
label card_quick_nap_scenario:
    scene bg vn-1

    show mi

    mi "The afternoon sun is making me feel so drowsy..."

    mi "I've been sitting at my computer for hours, mindlessly scrolling through the same websites."

    mi "My eyes feel heavy, and that warm sunlight streaming through the window is so inviting."

    mi "Maybe just a quick nap on the couch wouldn't hurt."

    mi "The cushions look so soft and comfortable right now."

    mi "I can hear the gentle hum of the air conditioning and distant sounds from outside."

    mi "Just twenty minutes... that's all I need."

    mi "Though knowing me, it'll probably turn into three hours."

    mi "But that's okay. Time doesn't really matter when you're not going anywhere."

    mi "The dust motes dancing in the sunbeam are almost hypnotic."

    mi "It's like they're inviting me to just... let go for a while."

    mi "Sometimes the best part of the day is when you can just stop thinking."

    mi "Stop worrying about all the things you should be doing."

    mi "And just sink into this warm, golden moment."

    hide mi

    "Mi settles into the couch, pulling a soft blanket over her shoulders."

    "The afternoon light grows softer as clouds drift across the sun."

    "Within minutes, gentle breathing fills the quiet living room."

    "The outside world continues its busy pace, but here, time moves differently."

    "In this peaceful bubble, even a simple nap becomes a small escape from everything."
