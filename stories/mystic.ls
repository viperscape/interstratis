tavern-enter
    if !player.visited_tavern "Welcome to the tavern"
    or "The tavern door swings open"

    if player.drunk "Back so soon? (bar tender)"

    next:now tavern
;

tavern
    @player.visited_tavern true
    next:select {"Have a drink" bar,
                "Look around" tavern-look,
                "Leave" tavern-exit}

    next:restart
;

tavern-look
    emit "You see a bartender, taking her time to make drinks. She seems to have a sour face on."
    if player.drunk ["You see a cloaked figure sitting in the corner."
       "Talk to cloaked figure?" next:await tavern-cloaked-figure]

    next:now tavern
;

tavern-cloaked-figure
    emit "Keep it to yourself, fool!"
    next:now tavern
;

tavern-exit
    emit "See ya, stranger (bar tender)"
    next:now town
;

town
    emit "You see a small tavern"
    next:select {"Sleep?" sleep,
                "Enter tavern?" tavern-enter}

    next:restart
;

sleep
    next:exit

;

root
    emit ["Welcome to the story about the Mystic"
         "You see a tavern with lights on and decide to enter"]
    next:now tavern-enter
;

bar
    emit "You take a gulp of a stiff drink" "You immediately feel the effects"
    @player.drunk true
    next:now tavern
;