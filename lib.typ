#let kth = (
  colors: (
    white: rgb(255, 255, 255),
    sand: rgb(235, 229, 224),
    blue: rgb(0, 71, 145),
    sky: rgb(98, 152, 210),
    navy: rgb(0, 0, 97),
    light-blue: rgb(222, 240, 255),
    digital-blue: rgb(0, 41, 237),
    dark-green: rgb(13, 74, 33),
    green: rgb(77, 160, 97),
    light-green: rgb(199, 235, 186),
    dark-turquoise: rgb(28, 67, 76),
    turquoise: rgb(51, 156, 156),
    light-turquoise: rgb(178, 224, 224),
    dark-brick: rgb(120, 0, 26),
    brick: rgb(232, 106, 88),
    light-brick: rgb(255, 204,  196),
    dark-yellow: rgb(166, 89, 0),
    yellow: rgb(255, 190, 0),
    light-yellow: rgb(255, 240, 176),
    dark-grey: rgb(50, 50, 50),
    grey: rgb(165, 165, 165),
    light-grey: rgb(230, 230, 230),
    broken-black: rgb(33, 33, 33),
    broken-white: rgb(252, 252, 252),
  ),
  fonts: (
    sans: "Figtree",
    serif: "Georgia",
    monospace: "JetBrains Mono Nerd Font",
  ),
  text-size: (
    cover-title: 24pt,
    title: 20pt,
    h1: 16pt,
    h2: 14pt,
    h3: 12pt,
    body: 11pt,
    small: 9pt,
  ),
  text-spacing: (
    cover-title: 25.4pt,
    title: 22pt,
    h1: 19.2pt,
    h2: 16.8pt,
    h3: 14.4pt,
    body: 13.2pt,
    small: 10pt,
  ),
);


#let problem(number, body) = {
  block(width: 100%, stroke: kth.colors.blue, inset: 0pt, radius: 0pt)[
    // Header
    #block(width: 100%, fill: kth.colors.blue, inset: 10pt)[
      #text(
        fill: kth.colors.white,
        weight: "bold",
        size: kth.text-size.h3,
        spacing: kth.text-spacing.h3,
        font: kth.fonts.sans
      )[Problem #number}]
    ]

    // Body
    #block(width: 100%, fill: kth.colors.sand, inset: 10pt, radius: 0pt)[
      #text(
        font: kth.fonts.sans,
        size: kth.text-size.body,
        spacing: kth.text-spacing.body,
        fill: black
      )[#body]
    ]
  ]
}

#let subproblem(letter, body) = {
  block(width: 100%, radius: 0pt, inset: 10pt)[
    #text(font: kth.fonts.sans, size: kth.text-size.body, spacing: kth.text-spacing.body)[#letter]
    #text(font: kth.fonts.sans, size: kth.text-size.body, spacing: kth.text-spacing.body)[#body]
  ]
}
