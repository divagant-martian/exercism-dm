class Phrase(phraseP: String) {
  var phrase: String = phraseP

  def wordCount(): Map[String, Int] = {
    phrase
      .toLowerCase()
      .map{(c) => {
        if (c.isLetterOrDigit || c=='\'')
          c
        else
          ' '
      }}
  }
}
