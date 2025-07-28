# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_emergency_medical_visit_scenario

# Emergency Medical Visit Card Scenario - "Medical Crisis"
label card_emergency_medical_visit_scenario:
    scene vn-26

    show mi

    mi "I can't ignore this anymore. Something is seriously wrong."

    mi "The chest pain, the dizziness, the way my heart races for no reason."

    mi "I kept telling myself it was just anxiety, just stress."

    mi "But sitting here in this waiting room, I realize how badly I've been treating my body."

    mi "The fluorescent lights are too bright after months of dim rooms."

    mi "All these healthy-looking people around me make me feel like an alien."

    mi "When did I become someone who avoids medical care until it's an emergency?"

    mi "The forms ask when I last saw a doctor. I honestly can't remember."

    mi "My isolation has extended to basic health maintenance too."

    mi "The nurse calls my name and I feel like I'm walking to judgment."

    mi "How do I explain that I've been living like a ghost for months?"

    mi "That I've survived on junk food and neglect?"

    mi "The doctor's going to take one look at me and know I've given up on self-care."

    mi "But I'm here now. That has to count for something."

    mi "Maybe this crisis will be the wake-up call I needed."

    mi "Maybe facing my physical health will help me face everything else too."

    hide mi

    "Mi sits nervously in the clinical environment, surrounded by the reality of medical care."

    "The sterile setting feels alien after months of isolation."

    "But seeking help, even in crisis, is a step toward taking responsibility."

    "Sometimes it takes a health scare to realize how precious and fragile our bodies are."

    "Today might be the beginning of treating herself like someone worth caring for."
