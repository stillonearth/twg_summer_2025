# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_channel_surf_scenario

# Channel Surf Card Scenario - "Mindless Clicking"
label card_channel_surf_scenario:
    scene vn-12

    show mi

    mi "Click. Nothing interesting. Click. Boring commercial. Click."

    mi "How many times can I cycle through these same channels?"

    mi "News... too depressing. Reality TV... too fake. Movies... all seen before."

    mi "My thumb moves on autopilot, pressing the channel button every few seconds."

    mi "There are supposedly hundreds of channels, but somehow nothing to watch."

    mi "Click. Shopping channel. Click. Foreign language news. Click. Static."

    mi "Even the static is more honest than most of what's on TV."

    mi "At least it doesn't pretend to be entertaining."

    mi "I could turn off the TV and do something productive."

    mi "But that would require making a decision, and right now my brain is in neutral."

    mi "Click. Cooking show. They're making something I'll never cook."

    mi "Click. Sports. People running around chasing balls. Why do people care?"

    mi "Click. Documentary about penguins. At least penguins have purpose."

    mi "Maybe I should get up, stretch, go outside, live life."

    mi "But instead... click."

    mi "The remote has become an extension of my hand."

    mi "Each click a tiny hope that the next channel will save me from this boredom."

    hide mi

    "The clicking continues, a rhythmic soundtrack to emptiness."

    "Images flash by on the screen without registering in her mind."

    "Time passes in clicks and channel changes."

    "Eventually, even the act of clicking becomes too much effort."

    "The remote falls from her hand as she stares at whatever random channel it landed on."
