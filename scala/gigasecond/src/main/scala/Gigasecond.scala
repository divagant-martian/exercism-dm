import java.time.LocalDate
import java.time.LocalDateTime
import java.time.format.DateTimeFormatter

object Gigasecond {

  private val gigaseconds: Int = Math.pow(10,9).toInt

  def addGigaseconds(input: LocalDateTime): LocalDateTime = {
    input.plusSeconds(gigaseconds)
  }

  def addGigaseconds(input: LocalDate): LocalDateTime = {
   input.atStartOfDay.plusSeconds(gigaseconds)
  }
}
