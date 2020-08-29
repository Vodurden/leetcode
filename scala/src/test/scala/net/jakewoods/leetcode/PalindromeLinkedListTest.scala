package net.jakewoods.leetcode

import org.scalacheck.Prop
import org.scalacheck.Arbitrary.arbitrary
import utest._
import utest.UTestScalaCheck

import net.jakewoods.leetcode.support.ListNode

object PalindromeLinkedListTest extends TestSuite with UTestScalaCheck {
  import PalindromeLinkedList._

  case class TestCase(input: ListNode, expected: Boolean) {
    def test(f: ListNode => Boolean) = {
      val result = f(input.deepClone)
      Predef.assert(
        result == expected,
        s"input: ${input}, palindrome: ${result}, expected: ${expected}"
      )
      assert(result == expected)
    }
  }

  val falseCase = TestCase(ListNode.listOf(1,2), false)
  val trueCase = TestCase(ListNode.listOf(1,2,2,1), true)
  val trueCaseOddLength = TestCase(ListNode.listOf(1,2,3,2,1), true)
  val singleValue = TestCase(ListNode.listOf(1), true)
  val duoValue = TestCase(ListNode.listOf(5,5), true)

  val tests = Tests {
    test("isPalindromeNaive") {
      test("false case") - falseCase.test(isPalindromeNaive)
      test("true case") - trueCase.test(isPalindromeNaive)
      test("true case odd length") - trueCaseOddLength.test(isPalindromeNaive)
      test("single value") - singleValue.test(isPalindromeNaive)
      test("duo value") - duoValue.test(isPalindromeNaive)
    }

    test("isPalindromeFast") {
      test("false case") - falseCase.test(isPalindromeFast)
      test("true case") - trueCase.test(isPalindromeFast)
      test("true case odd length") - trueCaseOddLength.test(isPalindromeFast)
      test("single value") - singleValue.test(isPalindromeFast)
      test("duo value") - duoValue.test(isPalindromeFast)
    }

    test("all algorithms have same input/output mapping") {
      Prop.forAll(arbitrary[ListNode]) { list =>
        val naiveResult = isPalindromeNaive(list)
        val fastResult = isPalindromeFast(list)
        naiveResult == fastResult
      }.checkUTest()
    }
  }
}
