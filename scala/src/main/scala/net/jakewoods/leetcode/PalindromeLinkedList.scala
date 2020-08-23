package net.jakewoods.leetcode

import net.jakewoods.leetcode.support.ListNode

object PalindromeLinkedList {

  /** Given a singly linked list, determine if it is a palindrome.
    *
    * Follow up: Could you do it in O(n) time and O(1) space?
    */
  def isPalindromeNaive(head: ListNode): Boolean = {
    import scala.collection.mutable.Stack

    val stack = Stack[Int]()

    var current = head
    while (current != null) {
      stack.push(current.x)
      current = current.next
    }

    current = head
    while (current != null) {
      val value = stack.top
      stack.pop()
      if(current.x != value) {
        return false
      }
      current = current.next
    }

    true
  }
}
