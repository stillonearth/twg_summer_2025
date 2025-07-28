# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_watch_news_scenario

# Watch News Card Scenario - "Staying Informed"
label card_watch_news_scenario:
    scene vn-13

    show mi

    mi "I should probably know what's happening in the world, right?"

    mi "Even if I'm not really part of it anymore."

    mi "The news anchor's voice is serious, urgent. Everything is breaking news these days."

    mi "Crisis here, disaster there, political drama everywhere."

    mi "Why do I do this to myself? I know it's going to make me feel worse."

    mi "But somehow I feel guilty if I don't stay informed."

    mi "Like I'm failing as a citizen or something."

    mi "The red breaking news banner flashes across the screen."

    mi "Another tragedy, another reason to lose faith in humanity."

    mi "The coffee in my cup is getting cold as I stare at the screen."

    mi "All these problems that feel so big, so overwhelming."

    mi "And here I am, hiding in my apartment, unable to even manage my own life."

    mi "How can anyone expect me to care about global issues?"

    mi "But I do care. That's the problem."

    mi "I care too much about things I can't control."

    mi "The weight of the world's problems settles on my shoulders."

    mi "Maybe ignorance really is bliss."

    hide mi

    "Mi continues watching despite the growing knot in her stomach."

    "Each news story adds another layer of anxiety and helplessness."

    "The outside world feels more distant and dangerous than ever."

    "By the time the weather forecast comes on, her mood has darkened considerably."

    "Staying informed feels more like staying depressed."
