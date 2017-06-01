def root
    title "interstratis"
    title_tag "h3"
    desc "interactive adventures"
    about "about"
;


root
    @root.title (tag) root.title_tag
    @root.desc (tag) "h4"
    @root.about (link) "https://github.com/viperscape/interstratis" "target='_blank'"
    
    emit [root.title
         root.desc
         root.about
         "<br></br>"]

;