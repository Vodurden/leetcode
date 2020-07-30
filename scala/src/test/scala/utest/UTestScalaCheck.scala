package utest

import org.scalacheck.{Test, Prop}
import org.scalacheck.util.Pretty

/** Integration trait between uTest and ScalaCheck
  *
  * Example usage:
  *
  *     object StringTests extends TestSuite with UTestScalaCheck {
  *       val tests = Tests {
  *         test("startsWith") {
  *           Prop.forAll { (a: String, b: String) =>
  *             (a + b).startsWith(b)
  *           }.checkUTest()
  *         }
  *       }
  *     }
  *
  * Adapted from: https://github.com/lihaoyi/utest/issues/2#issuecomment-67300735
  *
  */
trait UTestScalaCheck {
  protected[this] object UTestReporter extends Test.TestCallback {
    private val prettyParams = Pretty.defaultParams

    override def onTestResult(name: String, res: org.scalacheck.Test.Result) = {
      val scalaCheckResult = if (res.passed) "" else Pretty.pretty(res, prettyParams)
      assert(scalaCheckResult.isEmpty)
    }
  }

  implicit protected[this] class PropWrapper(prop: Prop) {
    def checkUTest(): Unit = {
      Test.check(prop) { params =>
        params.withTestCallback(UTestReporter)
      }
      ()
    }
  }
}
