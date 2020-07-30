package net.jakewoods.leetcode

import org.scalacheck.Prop
import org.scalacheck.Arbitrary.arbitrary
import utest._
import utest.UTestScalaCheck

import net.jakewoods.leetcode.support.ListNode

object ReverseLinkedListTest extends TestSuite with UTestScalaCheck {
  import ReverseLinkedList._

  case class TestCase(input: ListNode, output: ListNode) {
    def test(f: ListNode => ListNode) = {
      val result = f(input.deepClone)
      val expected = output // Re-assignment so `assert` prints the value
      assert(result == expected)
    }
  }

  val example1 = TestCase(ListNode.listOf(1,2,3,4,5), ListNode.listOf(5,4,3,2,1))
  val singleValue = TestCase(ListNode.listOf(2,3,4,4), ListNode.listOf(4,4,3,2))
  val duplicates = TestCase(ListNode.listOf(2,3,4,4), ListNode.listOf(4,4,3,2))
  val bigSmall = TestCase(ListNode.listOf(1085180431,-1), ListNode.listOf(-1, 1085180431))
  val singleBigValue = TestCase(ListNode.listOf(1856164307), ListNode.listOf(1856164307))

  def testInputOutputSameLengthProperty(f: ListNode => ListNode) = {
    Prop.forAll(arbitrary[ListNode]) { list =>
      val result = f(list.deepClone)
      list.length == result.length
    }.checkUTest()
  }

  val tests = Tests {
    test("reverseListIterative") {
      test("example 1") - example1.test(reverseListIterative)
      test("single value") - singleValue.test(reverseListIterative)
      test("duplicates") - duplicates.test(reverseListIterative)
      test("big small") - bigSmall.test(reverseListIterative)
      test("single big value") - singleBigValue.test(reverseListIterative)

      test("input and output length should be identical") - testInputOutputSameLengthProperty(reverseListIterative)
    }

    test("reverseListRecursive") {
      test("example 1") - example1.test(reverseListRecursive)
      test("single value") - singleValue.test(reverseListRecursive)
      test("duplicates") - duplicates.test(reverseListRecursive)
      test("big small") - bigSmall.test(reverseListRecursive)
      test("single big value") - singleBigValue.test(reverseListRecursive)

      test("input and output length should be identical") - testInputOutputSameLengthProperty(reverseListRecursive)
    }

    test("solutions are equivalent") {
      Prop.forAll(arbitrary[ListNode]) { list =>
        val iterativeResult = reverseListIterative(list.deepClone)
        val recursiveResult = reverseListRecursive(list.deepClone)
        iterativeResult == recursiveResult
      }.checkUTest()
    }
  }
}
