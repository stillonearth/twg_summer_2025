# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_rage_quit_scenario

# Rage Quit Card Scenario - "Gaming Frustration"
label card_rage_quit_scenario:
    scene vn-17

    show mi

    mi "GAME OVER. Again. For the fifteenth time on this same stupid level."

    mi "This is ridiculous! The hit detection is clearly broken!"

    mi "I had that jump perfect, but somehow I still fell into the pit."

    mi "These games are supposed to be fun, not torture devices."

    mi "My hands are cramping from gripping the controller so tightly."

    mi "And now it's mocking me with that cheerful 'Try Again!' message."

    mi "Try again? I've been trying again for two hours!"

    mi "This boss fight is impossible. The developers clearly hate their players."

    mi "One more attempt. Just one more..."

    mi "No. NO! That was even worse than last time!"

    mi "That's it. I'm done."

    mi "The controller goes flying across the room."

    mi "I hope it breaks. This stupid game deserves a broken controller."

    mi "Gaming is supposed to be my escape, not another source of stress."

    mi "Why can't I even succeed at something that's supposed to be relaxing?"

    mi "Even in virtual worlds, I'm a failure."

    hide mi

    "The controller lands with a satisfying thud against the wall."

    "The game over screen continues to glow mockingly on the TV."

    "Mi storms away from the gaming setup, heart racing with frustration."

    "Sometimes even escapism becomes another thing to escape from."

    "The gaming equipment sits abandoned, waiting for her mood to cool down."
