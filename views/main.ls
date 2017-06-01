def root
    title "interstratis"
    desc "interactive adventures"

;


root
    @root.title (tag) h3
    @root.desc (tag) h4
    
    emit [root.title
         root.desc]

;