# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_basic_hygiene_scenario

# Basic Hygiene Card Scenario - "Morning Routine"
label card_basic_hygiene_scenario:
    scene vn-8

    show mi

    mi "Okay, let's try to start the day right for once."

    mi "The face looking back at me in the mirror needs some attention."

    mi "When did basic hygiene become such a monumental task?"

    mi "But here I am, toothbrush in hand, ready to tackle the minimum requirements of human maintenance."

    mi "The mint flavor is sharp and wake-up-worthy."

    mi "Two minutes of brushing. Such a simple thing, but it feels like an accomplishment."

    mi "Now for washing my face with actual soap and water."

    mi "The cool water is refreshing against my skin."

    mi "I can feel the oil and sleep washing away."

    mi "Maybe this is what normal people do every single day without thinking about it."

    mi "But for me, today, this feels like a small victory."

    mi "Clean teeth, clean face. The basics are covered."

    mi "The person in the mirror looks a little more human now."

    mi "A little more like someone who might actually leave the house someday."

    mi "Baby steps, I guess. But steps nonetheless."

    mi "Maybe tomorrow I'll even comb my hair."

    hide mi

    "Mi tidies up the bathroom counter, putting things back in their places."

    "The morning light filters through the small window, casting clean shadows."

    "It's a small routine, but routines have power."

    "Sometimes the path back to normalcy starts with something as simple as brushing your teeth."

    "Today feels like it might actually be a day, not just time passing."

    jump card_complete
