# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_crash_sleep_scenario

# Crash Sleep Card Scenario - "Exhaustion Collapse"
label card_crash_sleep_scenario:
    scene vn-20

    show mi

    mi "I can't... I can't keep my eyes open anymore."

    mi "When did everything become so heavy?"

    mi "My eyelids feel like they weigh a thousand pounds each."

    mi "The room is spinning slightly, or maybe that's just me swaying."

    mi "How many days has it been since I really slept?"

    mi "Not just dozed off at my computer, but actual, proper sleep."

    mi "My body is staging a rebellion against my terrible habits."

    mi "The floor looks surprisingly comfortable right now."

    mi "Maybe if I just sit down for a second..."

    mi "No, that's a bad idea. If I sit, I'll fall over."

    mi "But standing is becoming impossible."

    mi "Everything feels distant and fuzzy, like I'm underwater."

    mi "My legs are giving out... I need to..."

    mi "Just need to... rest..."

    hide mi

    "Mi's body finally surrenders to the exhaustion she's been fighting."

    "She collapses more than lies down, consciousness fading immediately."

    "The room falls silent except for her deep, unconscious breathing."

    "Items scattered around her tell the story of someone who pushed too far."

    "Sleep takes her completely - not peaceful rest, but the desperate unconsciousness of a body at its limit."

    "Hours pass in dreamless void before her body begins to recover."

    "When she eventually wakes, disoriented and aching, it will be in a different day entirely."
