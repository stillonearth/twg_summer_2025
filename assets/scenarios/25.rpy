# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_procrastinate_everything_scenario

# Procrastinate Everything Card Scenario - "Avoidance Spiral"
label card_procrastinate_everything_scenario:
    scene vn-25

    show mi

    mi "Look at all these things I should be doing."

    mi "Bills to pay, emails to answer, appointments to schedule."

    mi "The pile just keeps growing like some kind of responsibility monster."

    mi "But today? Today I choose to ignore all of it."

    mi "Tomorrow. I'll definitely handle everything tomorrow."

    mi "That's what I said yesterday, and the day before that."

    mi "The calendar mocks me with its red deadlines and overdue reminders."

    mi "But the beauty of procrastination is that it makes everything a future problem."

    mi "Future Me can deal with this mess. Present Me is busy avoiding."

    mi "Each task I ignore feels like a small rebellion against adult responsibilities."

    mi "Who decided we all had to be so productive anyway?"

    mi "Maybe if I avoid something long enough, it will solve itself."

    mi "Or become someone else's problem. That works too."

    mi "The guilt is building up like storm clouds, but I'll deal with that tomorrow too."

    mi "For now, I'm going to pretend none of this exists."

    mi "Denial is my superpower, and I'm really good at it."

    mi "Tomorrow will have to be the most productive day in human history to catch up."

    hide mi

    "Mi deliberately turns away from her desk full of ignored responsibilities."

    "Each avoided task adds weight to the growing pile of 'tomorrow' problems."

    "The cycle of avoidance deepens, making everything feel more overwhelming."

    "But for now, in this moment, the problems don't exist if she doesn't look at them."

    "Tomorrow will arrive eventually, but today is for perfect, blissful avoidance."
