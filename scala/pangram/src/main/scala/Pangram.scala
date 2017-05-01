object Pangrams {
  def isPangram(sent: String): Boolean = {
    ('a' to 'z').forall((c) => {
      sent.toLowerCase().contains(c)
    })
  }
}
