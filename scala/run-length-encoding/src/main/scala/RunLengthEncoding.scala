object RunLengthEncoding {

  def encode(input: String): String = {
    input.foldLeft(("", ' ', 0, 1)) {
      (big_acc, c) => {
        //big_acc = (acc, last, last_n, pos)
        if (c == big_acc._2) {
          if (big_acc._4 == input.length()) { // end of string
            ( big_acc._1 + (big_acc._3 + 1) + c,
              big_acc._2,
              big_acc._3 + 1,
              big_acc._4 + 1 )
          }
          else {
            ( big_acc._1, big_acc._2, big_acc._3 + 1, big_acc._4 + 1 )
          }
        }
        else {
          var acc = big_acc._1
          if (big_acc._3 > 1) {
            acc += big_acc._3;
          }
          if (big_acc._3 > 0) { // ignore initial last (single whitespace)
            acc += big_acc._2;
          }
          if (big_acc._4 == input.length()) { // end of string
            acc += c;
          }
          (acc, c, 1, big_acc._4 + 1)
        }
      }
    }._1
  }

  def decode(input: String): String = {
    input
      .foldLeft(("", 0)) { (big_acc, c) => {
        if (Character.isDigit(c)) { // keep reading.. keeeep reading
          (big_acc._1, 10*big_acc._2 + ("" + c).toInt)
        }
        else {
          (big_acc._1 + (""+c)*math.max(big_acc._2, 1), 0)
        }
      }
    }._1
  }
}
