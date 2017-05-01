case class SpaceAge(s: Long) {
  val seconds = s;
  private val exactEarth = seconds/31557600.0
  lazy val onEarth = round(exactEarth)
  lazy val onMercury = round(exactEarth/0.2408467)
  lazy val onVenus = round(exactEarth/0.61519726)
  lazy val onMars = round(exactEarth/1.8808158)
  lazy val onJupiter = round(exactEarth/11.862615)
  lazy val onSaturn = round(exactEarth/29.447498)
  lazy val onUranus = round(exactEarth/84.016846)
  lazy val onNeptune = round(exactEarth/164.79132)
  private def round(x: Double): Double = Math.round(100*x)/100.0
}
