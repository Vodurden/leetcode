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

  def isPalindromeFast(head: ListNode): Boolean = {
    // Make the first half of the list point in the reverse direction. I.e.:
    //
    // - "1->2->3->2->1" becomes "1<-2  3->2->1"
    // - "1->2->2->1" becomes "1<-2  2->1
    //
    // Where `reversed` is the head of the reversed list and `remaining` is the
    // head of the remaining list

    // When `fast` hits null or the value before null `slow` will be in the middle
    // of the palindrome
    var slowPrevious: ListNode = null
    var slow = head
    var fast = head

    while(fast != null && fast.next != null) {
      fast = fast.next.next

      val oldSlow = slow.next
      slow.next = slowPrevious
      slowPrevious = slow
      slow = oldSlow
    }

    // slow is now "3->2->1" (from middle to end)
    // slowPrevious is now "2->1" (from middle to start)
    var midToStart = slowPrevious
    var midToEnd = slow

    // If `fast` is null we know the loop has an even length since fast advances two at a time.
    val evenLength = fast == null

    // If the list isn't even we should skip the middle node since it's equal to itself.
    //
    // This also means `midToEnd` and `midToStart` should always have the same length after
    // we apply this change
    if (!evenLength) {
      midToEnd = midToEnd.next
    }

    while (midToEnd != null && midToStart != null) {
      if(midToEnd.x != midToStart.x) {
        return false
      }

      midToEnd = midToEnd.next
      midToStart = midToStart.next
    }

    true
  }
}
