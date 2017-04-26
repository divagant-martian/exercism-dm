object Hamming {
  def compute(x:String, y:String): Option[Int] = {
    if (x.length() != y.length()) {None}
    else if (x == y) {Some(0)}
    else { Some((x zip y) count { x => x._1 != x._2 }) }
  }
}
