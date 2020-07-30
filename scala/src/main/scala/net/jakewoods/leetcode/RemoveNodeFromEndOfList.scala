package net.jakewoods.leetcode

import net.jakewoods.leetcode.support.ListNode

object RemoveNodeFromEndOfList {
  /** Given a linked list, remove the n-th node from the end of list and return its head.
    *
    * Note: Given n will always be valid.
    */
  def removeNthFromEndSinglePass(head: ListNode, n: Int): ListNode = {
    var nodes = Vector[ListNode]()
    var current = head
    while(current != null) {
      nodes = nodes.appended(current)
      current = current.next
    }

    nodes.lift(nodes.length - n - 1) match {
      case Some(nodeBeforeTarget) => {
        nodeBeforeTarget.next = nodeBeforeTarget.next.next
        head
      }
      case None => head.next
    }
  }

  def removeNthFromEndTwoPointer(head: ListNode, n: Int): ListNode = {
    // We prepend a node to avoid the "delete head" special case
    val start = new ListNode(-1)
    start.next = head

    var nodeBeforeTarget = start
    var lookahead = start

    // Make the gap between `nodeBeforeTarget` and `lookahead` equal to `n`
    for (_ <- 1 to n+1) {
      lookahead = lookahead.next
    }

    while (lookahead != null) {
      nodeBeforeTarget = nodeBeforeTarget.next
      lookahead = lookahead.next
    }

    nodeBeforeTarget.next = nodeBeforeTarget.next.next

    start.next
  }
}
