# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_order_delivery_scenario

# Order Delivery Card Scenario - "The Usual Order"
label card_order_delivery_scenario:
    scene vn-5

    show mi

    mi "Another evening, another delivery order."

    mi "The takeout menus are scattered across the table like old friends."

    mi "I know them all by heart now - every restaurant within delivery range."

    mi "The Thai place that always forgets the extra sauce I order."

    mi "The pizza joint that somehow always arrives exactly 45 minutes late."

    mi "And my usual - the Chinese restaurant that knows my order by voice."

    mi "I should feel embarrassed about that, but honestly, it's kind of nice."

    mi "At least someone recognizes me, even if it's just the delivery person."

    mi "The app glows on my phone screen, showing my order history."

    mi "Wow, I've really ordered from here twelve times this month."

    mi "I keep telling myself I'll cook tomorrow, but tomorrow never comes."

    mi "It's just so much easier to tap a few buttons and wait."

    mi "No shopping, no prep work, no dishes to clean afterward."

    mi "Just thirty minutes of waiting and then food appears at my door."

    mi "Like magic, if magic came with a delivery fee and tip."

    mi "The usual order it is, then."

    hide mi

    "Mi taps through the familiar app interface with practiced ease."

    "The order confirmation appears with an estimated delivery time."

    "She settles back into her routine of waiting, phone in hand."

    "Outside, the delivery drivers navigate the city streets, bringing convenience to doorsteps."

    "In thirty minutes, another meal will arrive, and the cycle continues."
