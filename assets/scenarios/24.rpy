# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_force_exercise_scenario

# Force Exercise Card Scenario - "Reluctant Workout"
label card_force_exercise_scenario:
    scene vn-24

    show mi

    mi "Okay, I'm going to do this. I'm actually going to exercise."

    mi "My body feels like it's made of concrete, but I have to start somewhere."

    mi "The workout equipment looks intimidating, like it's judging my fitness level."

    mi "Which is approximately zero, thanks for asking."

    mi "Every muscle in my body is protesting before I've even started."

    mi "But everyone says exercise helps with depression and anxiety."

    mi "So here I am, forcing myself through the motions."

    mi "Ten jumping jacks. My lungs are already burning."

    mi "How did I get this out of shape? Oh right, months of not moving."

    mi "Push-ups. More like push-downs. I'm basically just laying on the floor at this point."

    mi "My brain keeps telling me to quit, that this is pointless."

    mi "But I read somewhere that discipline beats motivation."

    mi "So even though I hate every second of this, I'm going to finish."

    mi "Sweat is already dripping, and I've barely done anything."

    mi "Maybe this is what progress feels like - terrible and necessary."

    mi "Just a few more minutes. I can survive a few more minutes."

    mi "My body might be weak, but I'm going to prove my will isn't."

    hide mi

    "Mi pushes through the workout despite every instinct telling her to stop."

    "Each movement is a small victory over inertia and self-doubt."

    "By the end, she's exhausted but oddly proud."

    "The endorphins haven't kicked in yet, but she did something positive for herself."

    "Tomorrow she might hate the idea again, but today she proved it's possible."

    jump card_complete

# Utility labels for card game system
label card_complete:
    "Card scenario completed!"
    return

# Additional system labels can be added here for other cards
# label card_reading_scenario:
# label card_music_scenario:
# etc.
