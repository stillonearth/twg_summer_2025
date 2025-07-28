# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_panic_eating_scenario

# Panic Eating Card Scenario - "Stress Binge"
label card_panic_eating_scenario:
    scene vn-33

    show mi

    mi "I can't stop. I know I should, but I can't."

    mi "The anxiety is eating me alive, so I'm eating everything else first."

    mi "Chips, cookies, leftover takeout - it all tastes like nothing."

    mi "But I keep shoving it in my mouth anyway."

    mi "Maybe if I fill this emptiness with food, it won't hurt so much."

    mi "Each bite is supposed to make me feel better, but it just makes me feel worse."

    mi "The packages pile up around me like evidence of my complete lack of control."

    mi "I'm not even hungry. Haven't been hungry for hours."

    mi "This isn't about food. It's about trying to feel something other than panic."

    mi "Or maybe trying to feel nothing at all."

    mi "My stomach hurts now, but I can't stop reaching for more."

    mi "This is what desperation tastes like - artificial flavoring and regret."

    mi "I know I'll hate myself for this later."

    mi "But later is later, and right now I need something, anything, to quiet the screaming in my head."

    mi "Food is the only comfort I can control, even when I can't control it at all."

    mi "The wrapper crinkles as I reach for yet another thing to consume."

    hide mi

    "Mi continues the destructive cycle, eating past fullness and comfort."

    "The kitchen becomes a battlefield of empty containers and self-loathing."

    "Each bite provides momentary distraction from overwhelming emotions."

    "But the relief is temporary, and the guilt compounds the original stress."

    "Eventually exhaustion wins over compulsion, leaving only the aftermath to face."
