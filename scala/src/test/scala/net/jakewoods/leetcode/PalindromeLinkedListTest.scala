package net.jakewoods.leetcode

import utest._

import net.jakewoods.leetcode.support.ListNode

object PalindromeLinkedListTest extends TestSuite {
  import PalindromeLinkedList._

  case class TestCase(input: ListNode, output: Boolean) {
    def test(f: ListNode => Boolean) = {
      val result = f(input.deepClone)
      val expected = output // Re-assignment so `assert` prints the value
      assert(result == expected)
    }
  }

  val falseCase = TestCase(ListNode.listOf(1,2), false)
  val trueCase = TestCase(ListNode.listOf(1,2,2,1), true)
  val singleValue = TestCase(ListNode.listOf(1), true)

  val tests = Tests {
    test("isPalindromeNaive") {
      test("false case") - falseCase.test(isPalindromeNaive)
      test("true case") - trueCase.test(isPalindromeNaive)
      test("single value") - singleValue.test(isPalindromeNaive)
    }
  }
}
