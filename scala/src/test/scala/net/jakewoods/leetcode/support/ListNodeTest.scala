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

    test("fold") {
      val input = ListNode.listOf(1,2,3)
      val result = input.fold(0) { case (acc, x) => acc + x }
      val expected = 6
      assert(result == expected)
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

    test("toString") {
      def check(list: ListNode, expected: String) = {
        val result = list.toString
        assert(result == expected)
      }

      test("List(0)") - check(ListNode.listOf(0), "0")
      test("List(1,2,3)") - check(ListNode.listOf(1,2,3), "1=>2=>3")
    }

    test("deepClone") {
      val original = ListNode.listOf(1,2,3)
      val cloned = original.deepClone

      cloned.next.x = 5

      val expectedOriginal = ListNode.listOf(1,2,3)
      assert(original == expectedOriginal)

      val expectedCloned = ListNode.listOf(1,5,3)
      assert(cloned == expectedCloned)
    }
  }
}
