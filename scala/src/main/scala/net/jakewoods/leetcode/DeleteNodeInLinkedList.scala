package net.jakewoods.leetcode

import net.jakewoods.leetcode.support.ListNode

object DeleteNodeInLinkedList {
  /** Write a function to delete a node (except the tail) in a singly linked list, given only access to that node.
   *
   *  Given linked list -- head = [4,5,1,9], which looks like following:
   *
   *      4 -> 5 -> 1 -> 9
   *
   *  Note:
   *
   *  - The linked list will have at least two elements.
   *  - All of the nodes' values will be unique.
   *  - The given node will not be the tail and it will always be a valid node of the linked list.
   *  - Do not return anything from your function.
   */
  def deleteNode(node: ListNode): Unit = {
    node.x = node.next.x
    node.next = node.next.next
  }
}
