# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_isolation_spiral_scenario

# Isolation Spiral Card Scenario - "Social Withdrawal"
label card_isolation_spiral_scenario:
    scene vn-30

    show mi

    mi "My phone has been buzzing all day. I'm not answering it."

    mi "Texts, calls, social invitations - they all feel like demands I can't meet."

    mi "People want things from me that I don't have to give."

    mi "Energy, conversation, the pretense of being okay."

    mi "It's easier to just... disappear for a while."

    mi "The curtains are drawn tight against the outside world."

    mi "In here, I don't have to perform being human for anyone."

    mi "No one can see how far I've fallen if I don't let them look."

    mi "Each ignored message makes the next one harder to answer."

    mi "Soon they'll stop trying, and honestly, that sounds like relief."

    mi "Social interaction feels like speaking a language I've forgotten."

    mi "What do normal people talk about? How do they make it look so easy?"

    mi "The silence in here is comfortable, predictable."

    mi "No unexpected questions, no judgment, no need to explain myself."

    mi "Maybe this is who I am now - someone who exists in the spaces between other people."

    mi "The notifications keep coming, but they feel like they're from another world."

    mi "A world I used to be part of but can't remember how to navigate."

    hide mi

    "Mi settles deeper into her self-imposed isolation."

    "The outside world continues its social rhythms without her."

    "Each ignored contact builds a higher wall between her and everyone else."

    "In the safety of solitude, she doesn't have to face her own struggles reflected in others' eyes."

    "But isolation, while protective, slowly becomes its own kind of prison."
