package net.jakewoods.leetcode

import net.jakewoods.leetcode.support.ListNode

object MergeTwoSortedLists {
  /** Merge two sorted linked lists and return it as a new sorted list. The new list should be made by splicing together the nodes of the first two lists.
    */
  def mergeTwoListsIterative(l1: ListNode, l2: ListNode): ListNode = {
    val result = new ListNode(-1)
    var current = result

    var lhsCurrent = l1
    var rhsCurrent = l2
    while(lhsCurrent != null && rhsCurrent != null) {
      if(lhsCurrent.x < rhsCurrent.x) {
        current.next = lhsCurrent
        lhsCurrent = lhsCurrent.next
      } else {
        current.next = rhsCurrent
        rhsCurrent = rhsCurrent.next
      }

      current = current.next
    }

    if (lhsCurrent == null) {
      current.next = rhsCurrent
    } else {
      current.next = lhsCurrent
    }

    result.next
  }

  def mergeTwoListsRecursive(l1: ListNode, l2: ListNode): ListNode = {
    if(l1 == null) { return l2 }
    if(l2 == null) { return l1 }

    if (l1.x < l2.x) {
      l1.next = mergeTwoListsRecursive(l1.next, l2)
      l1
    } else {
      l2.next = mergeTwoListsRecursive(l1, l2.next)
      l2
    }
  }
}
