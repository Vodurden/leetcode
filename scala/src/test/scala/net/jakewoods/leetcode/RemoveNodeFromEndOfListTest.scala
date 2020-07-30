package net.jakewoods.leetcode

import utest._

import net.jakewoods.leetcode.support.ListNode

object RemoveNodeFromEndOfListTest extends TestSuite {
  import RemoveNodeFromEndOfList._

  def makeTestSolution(f: (ListNode, Int) => ListNode)(input: ListNode, n: Int, expected: ListNode) = {
    val result = f(input, n)
    assert(result == expected)
  }

  val tests = Tests {
    test("removeNthFromEndSinglePass") {
      val testSolution = makeTestSolution(removeNthFromEndSinglePass) _

      test("example 1") - testSolution(ListNode.listOf(1,2,3,4,5), 2, ListNode.listOf(1,2,3,5))
      test("remove second value") - testSolution(ListNode.listOf(1,2,3,4,5), 4, ListNode.listOf(1,3,4,5))
      test("remove first") - testSolution(ListNode.listOf(1,2,3), 3, ListNode.listOf(2,3))
      test("remove last") - testSolution(ListNode.listOf(1,2,3), 1, ListNode.listOf(1,2))
      test("remove into singleton list") - testSolution(ListNode.listOf(1,2), 1, ListNode.listOf(1))
    }

    test("removeNthFromEndTwoPointer") {
      val testSolution = makeTestSolution(removeNthFromEndTwoPointer) _

      test("example 1") - testSolution(ListNode.listOf(1,2,3,4,5), 2, ListNode.listOf(1,2,3,5))
      test("remove second value") - testSolution(ListNode.listOf(1,2,3,4,5), 4, ListNode.listOf(1,3,4,5))
      test("remove first") - testSolution(ListNode.listOf(1,2,3), 3, ListNode.listOf(2,3))
      test("remove last") - testSolution(ListNode.listOf(1,2,3), 1, ListNode.listOf(1,2))
      test("remove into singleton list") - testSolution(ListNode.listOf(1,2), 1, ListNode.listOf(1))
    }
  }
}
