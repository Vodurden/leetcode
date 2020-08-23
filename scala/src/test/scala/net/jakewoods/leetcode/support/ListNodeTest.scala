package net.jakewoods.leetcode.support

import org.scalacheck.Prop
import org.scalacheck.Arbitrary.arbitrary
import utest._
import utest.UTestScalaCheck

object ListNodeTest extends TestSuite with UTestScalaCheck {
  val tests = Tests {
    test("equality") {
      Prop.forAll(arbitrary[ListNode]) { list =>
        list == list
      }.checkUTest()
    }

    test("listOf") {
      val result = ListNode.listOf(1, 2, 3)
      val expected = new ListNode(1)
      expected.next = new ListNode(2)
      expected.next.next = new ListNode(3)

      assert(result == expected)
    }

    test("foldNodes") {
      test("sum") {
        val input = ListNode.listOf(1,2,3)
        val result = input.foldNodes(0) { case (acc, _, node) => acc + node.x }
        val expected = 6
        assert(result == expected)
      }

      test("onCycle index is correct") {
        val input = ListNode.listOf(1,2).withCycle(0)
        val onCycle = (_: Int, index: Int, _: ListNode) => index
        val result = input.foldNodes(-1, onCycle) { case (acc, _, _) => acc }

        assert(result == 0)
      }

      test("onCycle deeper index is correct") {
        val input = ListNode.listOf(1,2,3,4).withCycle(2)
        val onCycle = (_: Int, index: Int, _: ListNode) => index
        val result = input.foldNodes(-1, onCycle) { case (acc, _, _) => acc }

        assert(result == 2)
      }
    }

    test("length") {
      def check(list: ListNode, expected: Int) = {
        val result = list.length
        assert(result == expected)
      }

      test("List(0)") - check(ListNode.listOf(0), 1)
      test("List(1,2,3)") - check(ListNode.listOf(1,2,3), 3)
      test("List(1085180431,-1)") - check(ListNode.listOf(1085180431,-1), 2)
      test("List(-1,1085180431)") - check(ListNode.listOf(1085180431,-1), 2)
    }

    test("toList") {
      def check(list: ListNode, expected: List[Int]) = {
        val result = list.toList
        assert(result == expected)
      }

      test("List(0)") - check(ListNode.listOf(0), List(0))
      test("List(1,2,3)") - check(ListNode.listOf(1,2,3), List(1,2,3))

      test("length matches") {
        Prop.forAll(arbitrary[ListNode]) { list =>
          list.length == list.toList.length
        }.checkUTest()
      }
    }

    test("at") {
      val List(a, _, _) = ListNode.nodes(1, 2, 3)
      val aAtZero = a.at(0)
      val expected = Option(a)
      assert(aAtZero == expected)
    }

    test("withCycle creates a cycle") {
      val List(a, b, c) = ListNode.nodes(1, 2, 3)
      a.withCycle(0)

      val aNext = a.next
      assert(aNext eq b)

      val bNext = b.next
      assert(bNext eq c)

      val cNext = c.next
      assert(cNext eq a)
    }

    test("toString") {
      def check(list: ListNode, expected: String) = {
        val result = list.toString
        assert(result == expected)
      }

      test("List(0)") - check(ListNode.listOf(0), "0")
      test("List(1,2,3)") - check(ListNode.listOf(1,2,3), "1=>2=>3")

      test("circular list") - check(ListNode.listOf(1,2).withCycle(0), "1=>2=>(Cycle to 0)")
    }

    test("deepClone") {
      test("normal list") - {
        val original = ListNode.listOf(1,2,3)
        val cloned = original.deepClone

        cloned.next.x = 5

        val expectedOriginal = ListNode.listOf(1,2,3)
        assert(original == expectedOriginal)

        val expectedCloned = ListNode.listOf(1,5,3)
        assert(cloned == expectedCloned)
      }

      test("circular list") - {
        val original = ListNode.listOf(1,2,3).withCycle(1)
        val cloned = original.deepClone

        cloned.x = 4
        cloned.next.x = 5
        cloned.next.next.x = 6
        cloned.next.next.next.x = 15

        val expectedOriginal = ListNode.listOf(1,2,3)
        assert(original == expectedOriginal)

        val expectedCloned = ListNode.listOf(4,15,6)
        assert(cloned == expectedCloned)
      }
    }
  }
}
