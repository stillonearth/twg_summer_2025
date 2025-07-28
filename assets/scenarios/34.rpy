# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_hide_under_blanket_scenario

# Hide Under Blanket Card Scenario - "Blanket Fortress"
label card_hide_under_blanket_scenario:
    scene vn-34

    show mi

    mi "If I can't see the world, maybe it can't see me either."

    mi "Under here, wrapped in layers of soft fabric, everything feels safer."

    mi "This is my fortress against reality."

    mi "The outside world with all its demands and expectations can't reach me here."

    mi "In this cocoon of blankets, I'm protected from everything that's overwhelming me."

    mi "The fabric muffles sounds, dims the light, creates a barrier between me and everything else."

    mi "I know this probably looks pathetic to anyone watching."

    mi "A grown person hiding under blankets like a scared child."

    mi "But sometimes you need to retreat to the most basic form of comfort."

    mi "Soft, warm, completely enveloping - like being held when no one else is here."

    mi "In here, I don't have to be functional or responsible or okay."

    mi "I can just exist in this small, controllable space."

    mi "The blankets smell like fabric softener and safety."

    mi "Maybe if I stay here long enough, the scary parts of life will just... go away."

    mi "Or maybe I'll find enough courage in this softness to face them later."

    mi "For now, this is enough. This tiny sanctuary is enough."

    hide mi

    "Mi disappears completely under the layers of blankets."

    "The room grows quiet except for her muffled breathing."

    "In this makeshift cocoon, the harsh edges of reality soften."

    "Sometimes the bravest thing you can do is create a safe space for yourself."

    "Under the blankets, she can exist without judgment, even from herself."
