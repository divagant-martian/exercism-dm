class Phrase(phraseP: String) {
  var phrase: String = phraseP

  def wordCount(): Map[String, Int] = {
    phrase
      .toLowerCase
      .map {(c) => {
        if (c.isLetterOrDigit || c=='\'') c
        else ' '
      }}
      .split(" +")
      .groupBy(identity)
      .mapValues(_.size)
      .toMap
  }
}
