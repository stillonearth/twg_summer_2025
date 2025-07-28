# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_answer_call_scenario

# Answer Call Card Scenario - "Breaking the Silence"
label card_answer_call_scenario:
    scene vn-31

    show mi

    mi "The phone is ringing. Again."

    mi "I've been staring at it for three rings now."

    mi "My finger hovers over the answer button like it's a live wire."

    mi "When did talking to people become so terrifying?"

    mi "But maybe... maybe I should try. Just this once."

    mi "My voice will sound rusty from disuse."

    mi "What if I have nothing interesting to say?"

    mi "What if they can hear the loneliness in my voice?"

    mi "Ring four. If I don't answer now, it goes to voicemail."

    mi "And then I'll spend the next hour analyzing why I couldn't do this simple thing."

    mi "Okay. Deep breath. Here goes nothing."

    mi "Hello?"

    mi "Oh. Hi. Yes, I'm... I'm okay."

    mi "Actually talking to another human being. When did this become an achievement?"

    mi "Their voice sounds so warm, so normal."

    mi "Like a reminder that the outside world still exists."

    mi "Maybe I can do this. Maybe connection doesn't have to be scary."

    hide mi

    "Mi's voice shakes slightly as she engages in actual conversation."

    "Each word spoken is a small bridge back to the world of human connection."

    "The caller's warmth begins to thaw something that had frozen inside her."

    "Breaking silence is harder than maintaining it, but infinitely more rewarding."

    "By the end of the call, she remembers what it feels like to be heard and understood."
