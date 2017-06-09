def meta
    # I define meta here because I can use it later, it will be inserted with name and id of story cache
;

root
    emit "Hello Interstratis!"
    next:now intro
;

intro
    emit [
       "Welcome"
       "Interstratis is about sharing stories"
       "Creating adventures"
       "Having fun"

       ]
;