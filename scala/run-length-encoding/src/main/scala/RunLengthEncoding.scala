object RunLengthEncoding {

  def encode(input: String): String = {
    input.foldLeft(("", ' ', 0, 1)) {
      case ((acc, last, last_n, pos), c) => {
        if (c == last)
          if (pos == input.length())
            (acc + (last_n + 1) + c, last, last_n + 1, pos + 1)
          else
            (acc, last, last_n + 1, pos + 1)
        else {
          var acc_t = acc
          if (last_n > 1) acc_t += last_n;
          if (last_n > 0) acc_t += last;
          if (pos == input.length()) acc_t += c;
          (acc_t, c, 1, pos + 1)
        }
      }
    }._1
  }

  def decode(input: String): String = {
    input
      .foldLeft(("", 0)) { case ((acc, last), c) => {
        if (Character.isDigit(c)) (acc, 10*last + ("" + c).toInt)
        else (acc + (""+c)*math.max(last, 1), 0)
      }
    }._1
  }
}
