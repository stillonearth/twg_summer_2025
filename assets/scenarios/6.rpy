# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_drink_water_scenario

# Drink Water Card Scenario - "Simple Hydration"
label card_drink_water_scenario:
    scene vn-6

    show mi

    mi "When did I last drink actual water?"

    mi "Not coffee, not soda, not energy drinks... just plain water."

    mi "My lips feel dry, and there's that familiar headache creeping in."

    mi "The glass feels cool and smooth in my hands."

    mi "Such a simple thing, but somehow I always forget."

    mi "The water is so clear and clean, with little droplets clinging to the outside of the glass."

    mi "I take a sip, and it's... refreshing."

    mi "Not exciting, not flavored, just pure and clean."

    mi "Why do I complicate everything when sometimes the simplest things are what I need?"

    mi "My body has been asking for this for hours, probably days."

    mi "But I kept reaching for everything else instead."

    mi "Another sip, and I can feel it working its way through my system."

    mi "Hydration. Such a basic human need, and yet..."

    mi "I treat it like an afterthought."

    mi "Maybe this is what self-care actually looks like."

    mi "Not grand gestures or expensive treatments."

    mi "Just remembering to give your body what it needs."

    hide mi

    "Mi finishes the glass of water slowly, mindfully."

    "For a moment, she feels more present in her own body."

    "The headache begins to fade, replaced by a subtle sense of clarity."

    "It's a small victory, but victories come in all sizes."

    "Sometimes taking care of yourself starts with something as simple as a glass of water."
