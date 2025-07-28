# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_grab_snacks_scenario

# Grab Snacks Card Scenario - "Guilty Snacking"
label card_grab_snacks_scenario:
    scene vn-4

    show mi

    mi "Here I am again, standing in front of the open fridge at midnight."

    mi "The cool blue light makes everything look so tempting."

    mi "I know I shouldn't be eating this stuff, but..."

    mi "Sometimes you just need that immediate hit of sugar and salt."

    mi "Those leftover chips from yesterday are calling my name."

    mi "And there's still some ice cream hidden behind the leftovers."

    mi "I tell myself it's just for tonight. Just this once."

    mi "But we both know that's not true, don't we?"

    mi "This has become part of my routine now."

    mi "Stay up late, feel restless, raid the kitchen for comfort food."

    mi "The crunch of the chips is so satisfying in the quiet house."

    mi "For a few minutes, the empty feeling inside gets filled with something."

    mi "Even if it's just artificial flavoring and preservatives."

    mi "I should probably buy healthier snacks next time I go shopping."

    mi "But let's be honest, when was the last time I actually went grocery shopping?"

    mi "Convenience store runs don't count."

    hide mi

    "Mi grabs an armful of snacks and closes the fridge door."

    "The kitchen returns to darkness, but the guilt lingers."

    "She knows this isn't solving anything, but sometimes coping is enough."

    "Tomorrow she'll promise to do better, eat healthier, take care of herself."

    "But tonight, these empty calories will have to fill the void."
