# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_morning_anxiety_scenario

# Morning Anxiety Card Scenario - "Dawn Dread"
label card_morning_anxiety_scenario:
    scene vn-22

    show mi

    mi "The sunlight is creeping through the blinds like an unwelcome visitor."

    mi "Morning. Another day I have to somehow get through."

    mi "My stomach is already churning with that familiar anxiety."

    mi "What terrible things will today bring?"

    mi "The light should feel warm and hopeful, but instead it feels accusatory."

    mi "Like it's highlighting everything I should be doing but won't."

    mi "My phone is buzzing with notifications I'm afraid to check."

    mi "Emails, messages, reminders of a world that keeps moving without me."

    mi "Even the birds outside sound too cheerful, too optimistic."

    mi "Don't they know that some of us aren't ready for another day?"

    mi "My heart is already racing and I haven't even gotten out of bed."

    mi "The sheets feel like the only safe place in the universe."

    mi "But I can't stay here forever, can I?"

    mi "Though honestly, forever sounds pretty good right about now."

    mi "Another day of pretending to be functional when I'm anything but."

    mi "Maybe if I pull the covers over my head, the world will go away."

    mi "Just for today. Just until I can figure out how to be human again."

    hide mi

    "Mi curls deeper into her blankets, trying to hide from the advancing day."

    "The morning light grows stronger despite her resistance."

    "Outside, the world begins its daily routine, indifferent to her struggle."

    "Every sunrise feels like a challenge she's not equipped to meet."

    "But somehow, she'll have to find a way to face whatever comes next."
