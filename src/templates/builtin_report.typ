// PREAMBLE
// The orcid image is supposed to be 24px, that is equivalent to 18pt
{{ORCID_ICON_DECLARATION}}

#set document(author: "{{AUTHOR_NAME}}")
#set text(lang: "{{LANG}}")
#set page()


#align(center)[
  TITLE HERE
  {{AUTHOR_NAME}}{{ORCID_ID}}
  #datetime.today().display()
]
#pagebreak()

// CONTENT
