# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_skip_hygiene_scenario

# Skip Hygiene Card Scenario - "Avoiding the Mirror"
label card_skip_hygiene_scenario:
    scene vn-9

    show mi

    mi "I can't... I just can't face the mirror today."

    mi "The toothbrush sits there on the counter, mocking me."

    mi "Such a simple thing, and yet it feels impossible right now."

    mi "I know I should brush my teeth, wash my face, do the basic human things."

    mi "But looking at myself in that mirror means confronting reality."

    mi "And today, reality feels too heavy to bear."

    mi "The person looking back at me would be a stranger anyway."

    mi "Hollow eyes, unkempt hair, the face of someone who's given up."

    mi "So I'll just... avoid it."

    mi "Turn away from the reflection and pretend it doesn't exist."

    mi "Maybe if I don't see myself, I can pretend I'm still the person I used to be."

    mi "The hygiene items sit there unused, gathering dust."

    mi "Tomorrow, I tell myself. Tomorrow I'll try again."

    mi "But we both know how often tomorrow turns into never."

    mi "Some days, just existing is exhausting enough."

    mi "Adding expectations on top of that feels cruel."

    hide mi

    "Mi shuffles out of the bathroom without a second glance."

    "The mirror remains mercifully ignored, its reflection empty."

    "Sometimes self-preservation means avoiding self-confrontation."

    "Not every day can be a victory, and that has to be okay too."

    "Tomorrow will come whether she's ready for it or not."
