class Bob {
  def hey(say:String):String = {
    val alph = ('a' to 'z') ++ ('A' to 'Z')

    if (say.trim().isEmpty()) {
      "Fine. Be that way!"
    }
    else if (say.toUpperCase().equals(say) && say.exists( alph contains _ )) {
      "Whoa, chill out!"
    }
    else if (say.endsWith("?")) {
      "Sure."
    }
    else {
      "Whatever."
    }
  }
}
