package net.jakewoods.leetcode

import utest._

import net.jakewoods.leetcode.support.ListNode

object DeleteNodeInLinkedListTest extends TestSuite {
  val tests = Tests {
    // Input: head = [4,5,1,9], node = 5
    // Output: [4,1,9]
    // Explanation: You are given the second node with value 5, the linked list should
    //               become 4 -> 1 -> 9 after calling your function.
    test("example 1") {
      val List(a, b, _, _) = ListNode.nodes(4,5,1,9)
      DeleteNodeInLinkedList.deleteNode(b)

      val result = a.toList
      val expected = List(4,1,9)
      assert(result == expected)
    }

    // Input: head = [4,5,1,9], node = 1
    // Output: [4,5,9]
    // Explanation: You are given the third node with value 1, the linked list should become 4 -> 5 -> 9 after calling your function.
    test("example 2") {
      val List(a, _, c, _) = ListNode.nodes(4,5,1,9)
      DeleteNodeInLinkedList.deleteNode(c)

      val result = a.toList
      val expected = List(4,5,9)
      assert(result == expected)
    }
  }
}
