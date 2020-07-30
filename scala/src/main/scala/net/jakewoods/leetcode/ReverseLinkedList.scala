package net.jakewoods.leetcode

import net.jakewoods.leetcode.support.ListNode

object ReverseLinkedList {
  /** Reverse a singly linked list.
    *
    * Follow up: A linked list can be reversed either iteratively or recursively. Could you implement both?
    */
  def reverseListIterative(head: ListNode): ListNode = {
    var previous: ListNode = null
    var current = head

    while(current != null) {
      val oldNext = current.next
      current.next = previous

      previous = current
      current = oldNext
    }

    previous
  }

  def reverseListRecursive(head: ListNode): ListNode = {
    if(head == null || head.next == null) { return head }

    val newHead = reverseListRecursive(head.next)
    head.next.next = head
    head.next = null
    newHead
  }
}
