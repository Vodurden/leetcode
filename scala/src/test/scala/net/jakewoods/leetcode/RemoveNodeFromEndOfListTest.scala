package net.jakewoods.leetcode

import utest._

import net.jakewoods.leetcode.support.ListNode

object RemoveNodeFromEndOfListTest extends TestSuite {
  import RemoveNodeFromEndOfList._

  case class TestCase(input: ListNode, n: Int, output: ListNode) {
    def test(f: (ListNode, Int) => ListNode) = {
      val result = f(input.deepClone, n)
      val expected = output // Re-assignment so `assert` prints the value
      assert(result == expected)
    }
  }

  val example1 = TestCase(ListNode.listOf(1,2,3,4,5), 2, ListNode.listOf(1,2,3,5))
  val removeSecondValue = TestCase(ListNode.listOf(1,2,3,4,5), 4, ListNode.listOf(1,3,4,5))
  val removeHead = TestCase(ListNode.listOf(1,2,3), 3, ListNode.listOf(2,3))
  val removeLast = TestCase(ListNode.listOf(1,2,3), 1, ListNode.listOf(1,2))
  val removeIntoSingletonList = TestCase(ListNode.listOf(1,2), 1, ListNode.listOf(1))

  val tests = Tests {
    test("removeNthFromEndSinglePass") {
      test("example 1") - example1.test(removeNthFromEndSinglePass)
      test("remove second value") - removeSecondValue.test(removeNthFromEndSinglePass)
      test("remove head") - removeHead.test(removeNthFromEndSinglePass)
      test("remove last") - removeLast.test(removeNthFromEndSinglePass)
      test("remove into singleton list") - removeIntoSingletonList.test(removeNthFromEndSinglePass)
    }

    test("removeNthFromEndTwoPointer") {
      test("example 1") - example1.test(removeNthFromEndTwoPointer)
      test("remove second value") - removeSecondValue.test(removeNthFromEndTwoPointer)
      test("remove head") - removeHead.test(removeNthFromEndTwoPointer)
      test("remove last") - removeLast.test(removeNthFromEndTwoPointer)
      test("remove into singleton list") - removeIntoSingletonList.test(removeNthFromEndTwoPointer)
    }
  }
}
