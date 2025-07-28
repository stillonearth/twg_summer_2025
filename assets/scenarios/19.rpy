# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_social_media_scenario

# Social Media Card Scenario - "Social Comparison"
label card_social_media_scenario:
    scene vn-19

    show mi

    mi "Why do I do this to myself?"

    mi "Every post is someone's perfect vacation, perfect relationship, perfect life."

    mi "Sarah just got promoted again. Mark is backpacking through Europe."

    mi "Here's Jenny at another fancy restaurant with friends I don't recognize."

    mi "And me? I'm sitting in pajamas that I haven't changed in three days."

    mi "Everyone seems to have it figured out except me."

    mi "Look at all these smiling faces in exotic locations."

    mi "Meanwhile, the most exotic place I've been this month is the convenience store downstairs."

    mi "Each scroll down feels like a reminder of everything I'm not doing."

    mi "Everyone's highlight reel compared to my behind-the-scenes disaster."

    mi "I should post something too, but what?"

    mi "A picture of my unmade bed? My collection of empty takeout containers?"

    mi "The contrast between their bright, filtered photos and my dim room is stark."

    mi "Maybe if I had a better camera angle, my life would look impressive too."

    mi "But who am I kidding? You can't Instagram your way out of isolation."

    mi "I close the app, but somehow it's open again five minutes later."

    mi "It's like picking at a scab that never quite heals."

    hide mi

    "Mi continues scrolling despite the growing knot in her stomach."

    "Each perfect post adds another layer to her feelings of inadequacy."

    "The bright social media interface glows mockingly in her dark room."

    "Other people's joy becomes a mirror reflecting her own emptiness."

    "She knows it's all curated illusion, but knowing doesn't make it hurt less."
