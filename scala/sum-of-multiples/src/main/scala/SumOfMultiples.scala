object SumOfMultiples {
  def sumOfMultiples(y: Set[Int], x: Int): Int = {
    (1 to x - 1).filter(
        (i) => y.exists(
            (j) =>  i%j == 0)).sum
  }
}
