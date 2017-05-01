object Squares {
  def difference(n: Int): Int = {
    (n-1)*n*(n+1)*(3*n+2)/12
  }

  def squareOfSums(n: Int): Int = {
    (Math.pow(n,2)*Math.pow(n+1,2)/4).toInt
  }

  def sumOfSquares(n: Int): Int = {
    n*(n+1)*(2*n+1)/6
  }
}
