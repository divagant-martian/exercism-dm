case class Year(y: Int) {
  var year = y
  def isLeap(): Boolean = {
    ( year%4 == 0 && year%100 != 0) || year%400 == 0
  }
}
