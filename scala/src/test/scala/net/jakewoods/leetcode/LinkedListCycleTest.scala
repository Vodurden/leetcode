package net.jakewoods.leetcode

import utest._

import net.jakewoods.leetcode.support.ListNode

object LinkedListCycleTest extends TestSuite {
  import LinkedListCycle._

  case class TestCase(input: ListNode, expected: Boolean) {
    def test(f: ListNode => Boolean) = {
      val result = f(input.deepClone)
      Predef.assert(
        result == expected,
        s"input: ${input}, cycle detected: ${result}, expected: ${expected}"
      )
    }
  }

  val longTrueCase = TestCase(ListNode.listOf(3,2,0,-4).withCycle(1), true)
  val smallTrueCase = TestCase(ListNode.listOf(1,2).withCycle(0), true)
  val minimalTrueCase = TestCase(ListNode.listOf(1).withCycle(0), true)
  val duplicateValuesTrueCase = TestCase(ListNode.listOf(1,1,2).withCycle(1), true)

  val exampleFalseCase = TestCase(ListNode.listOf(1,2,3), false)
  val singleNodeFalseCase = TestCase(ListNode.listOf(1), false)
  val duplicateValuesFalseCase = TestCase(ListNode.listOf(1,1,2), false)

  val tests = Tests {
    test("hasCycleWithSet") {
      test("long true case") - longTrueCase.test(hasCycleWithSet)
      test("small true case") - smallTrueCase.test(hasCycleWithSet)
      test("minimal true case") - minimalTrueCase.test(hasCycleWithSet)
      test("duplicate values true case") - duplicateValuesTrueCase.test(hasCycleWithSet)

      test("example false case") - exampleFalseCase.test(hasCycleWithSet)
      test("single node false case") - singleNodeFalseCase.test(hasCycleWithSet)
      test("duplicate values false case") - duplicateValuesFalseCase.test(hasCycleWithSet)
    }

    test("hasCycleNoAllocation") {
      test("long true case") - longTrueCase.test(hasCycleNoAllocation)
      test("small true case") - smallTrueCase.test(hasCycleNoAllocation)
      test("minimal true case") - minimalTrueCase.test(hasCycleNoAllocation)
      test("duplicate values true case") - duplicateValuesTrueCase.test(hasCycleNoAllocation)

      test("example false case") - exampleFalseCase.test(hasCycleNoAllocation)
      test("single node false case") - singleNodeFalseCase.test(hasCycleNoAllocation)
      test("duplicate values false case") - duplicateValuesFalseCase.test(hasCycleNoAllocation)
    }
  }
}
