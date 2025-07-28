# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_cook_meal_scenario

# Cook Meal Card Scenario - "Kitchen Therapy"
label card_cook_meal_scenario:
    scene vn-3

    show mi

    mi "For once, I actually feel like cooking something real."

    mi "Not just instant noodles or convenience store sandwiches."

    mi "These fresh vegetables on the counter... when did I last buy actual ingredients?"

    mi "The tomatoes are so red and firm. The onions have that sharp, clean smell."

    mi "There's something therapeutic about chopping vegetables."

    mi "The rhythmic sound of the knife on the cutting board is almost meditative."

    mi "Steam rises from the pan, and suddenly the kitchen feels alive."

    mi "I forgot how satisfying it is to create something with your own hands."

    mi "Even if it's just a simple meal that only I will eat."

    mi "The sizzling sound as the onions hit the hot oil..."

    mi "That golden color as they slowly caramelize..."

    mi "For these few minutes, my mind isn't wandering to dark places."

    mi "I'm focused on this one simple task."

    mi "Cooking doesn't judge you. It doesn't care if you're behind on life."

    mi "It just asks you to be present, to pay attention."

    mi "Maybe this is what people mean when they talk about mindfulness."

    hide mi

    "Mi continues cooking, losing herself in the familiar motions."

    "The kitchen fills with warm, inviting aromas."

    "For the first time in weeks, she feels a small sense of accomplishment."

    "Sometimes the simplest acts of self-care can feel like victories."

    "Tonight, Mi will eat a home-cooked meal, and that's enough."
