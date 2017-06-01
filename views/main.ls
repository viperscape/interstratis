def root
    title "interstratis"
    desc "interactive adventures"
    about "about"
;


root
    @root.title (tag) h3
    @root.desc (tag) h4
    @root.about (link) "https://github.com/viperscape/interstratis" "target='_blank'"
    
    emit [root.title
         root.desc
         root.about
         "<br></br>"]

;