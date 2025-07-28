# Hikikomori Card Game - Visual Novel Script
# Character and Image Definitions

define mi = Character("Mi", color="#c8ffc8")

# Main Menu/Card Selection (simplified for this episode)
label start:
    jump card_browse_internet_scenario

# Browse Internet Card Scenario - "Casual Browsing"
label card_browse_internet_scenario:
    scene vn-18

    show mi

    mi "Seventeen tabs open and counting."

    mi "How did I get from checking my email to reading about the mating habits of deep-sea creatures?"

    mi "The internet is like a maze designed by someone with severe ADHD."

    mi "Each click leads to three more interesting links."

    mi "I started researching something important for... what was it again?"

    mi "Oh right, apartment cleaning tips. Now I'm watching videos of cats falling off furniture."

    mi "This is productivity in the modern age, I guess."

    mi "Tab one: social media. Tab two: news. Tab three: random Wikipedia article about medieval torture devices."

    mi "You know, the usual Tuesday afternoon browsing."

    mi "My browser is probably crying under the weight of all these open pages."

    mi "But closing tabs feels like admitting defeat."

    mi "What if I need that article about 'Signs Your Houseplant is Plotting Against You' later?"

    mi "The blue glow of the screen has become my primary source of vitamin D."

    mi "Three hours have passed and I've accomplished absolutely nothing."

    mi "But I now know seventeen different ways to fold a fitted sheet."

    mi "Information without application. The internet's greatest gift and curse."

    hide mi

    "Mi continues clicking through an endless maze of hyperlinks."

    "Each new page promises to be the last one, but never is."

    "The computer screen flickers with the accumulated weight of digital wandering."

    "Time dissolves into an endless scroll of content and distraction."

    "By evening, she'll have learned a thousand useless facts and forgotten what she started looking for."
