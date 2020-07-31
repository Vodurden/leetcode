package net.jakewoods.leetcode

import org.scalacheck.Prop
import org.scalacheck.Arbitrary.arbitrary
import utest._
import utest.UTestScalaCheck

import net.jakewoods.leetcode.support.ListNode

object MergeTwoSortedListsTest extends TestSuite with UTestScalaCheck {
  import MergeTwoSortedLists._

  case class TestCase(input1: ListNode, input2: ListNode, output: ListNode) {
    def test(f: (ListNode, ListNode) => ListNode) = {
      val i1 = if(input1 != null) input1.deepClone else input1
      val i2 = if(input2 != null) input2.deepClone else input2
      val result = f(i1, i2)
      val expected = output // Re-assignment so `assert` prints the value
      assert(result == expected)
    }
  }

  val example1 = TestCase(ListNode.listOf(1,2,4), ListNode.listOf(1,3,4), ListNode.listOf(1,1,2,3,4,4))
  val emptyRhs = TestCase(ListNode.listOf(1), null, ListNode.listOf(1))
  val emptyLhs = TestCase(null, ListNode.listOf(1), ListNode.listOf(1))

  def testInputSumLengthEqualOutputLengthProperty(f: (ListNode, ListNode) => ListNode) = {
    val generator = for {
      input1 <- arbitrary[ListNode]
      input2 <- arbitrary[ListNode]
    } yield (input1, input2)

    Prop.forAll(generator) { case (input1, input2) =>
      val resultLength = f(input1.deepClone, input2.deepClone).length
      val input1Length = input1.length
      val input2Length = input2.length

      // Assert so the input values are printed on error
      assert(resultLength == input1Length + input2Length)

      resultLength == input1Length + input2Length
    }.checkUTest()
  }

  def testSolution(input1: ListNode, input2: ListNode, output: ListNode) = {
    val result = mergeTwoListsRecursive(input1, input2)
    val expected = output
    assert(result == expected)
  }

  val tests = Tests {
    test("mergeTwoListsIterative") {
      test("example 1") - example1.test(mergeTwoListsIterative)
      test("empty rhs") - emptyRhs.test(mergeTwoListsIterative)
      test("empty lhs") - emptyLhs.test(mergeTwoListsIterative)
      test("length of both inputs should sum to the output length") - testInputSumLengthEqualOutputLengthProperty(mergeTwoListsIterative)
    }

    test("mergeTwoListsRecursive") {
      test("example 1") - example1.test(mergeTwoListsRecursive)
      test("empty rhs") - emptyRhs.test(mergeTwoListsRecursive)
      test("empty lhs") - emptyLhs.test(mergeTwoListsRecursive)
      test("length of both inputs should sum to the output length") - testInputSumLengthEqualOutputLengthProperty(mergeTwoListsRecursive)
    }
  }
}
