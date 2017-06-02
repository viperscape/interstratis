def header
    title "interstratis"
    title_tag "h3"
    desc "interactive adventures"
    desc_tag "h4"
;

def footer
    about "about"
    
    link "https://github.com/viperscape/interstratis"
    link_target "target='_blank'"

;


root
    @header.title (tag) header.title_tag
    @header.desc (tag) header.desc_tag
    @footer.about (link) footer.link footer.link_target
    
    emit [header.title
         header.desc]

    emit stories.story

    emit footer.about

;