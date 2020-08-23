package net.jakewoods.leetcode

import net.jakewoods.leetcode.support.ListNode

object LinkedListCycle {
/** Given a linked list, determine if it has a cycle in it.
  */
  def hasCycleWithSet(head: ListNode): Boolean = {
    // IdentityHashMap is similar to a HashMap except it uses reference equality
    // instead of value equality for the key. It's not very useful in most cases
    // but it's perfect here since we don't want to compare ListNode's by value
    val visited = new java.util.IdentityHashMap[ListNode, Unit]()

    var current = head
    while (current != null) {
      if (visited.containsKey(current)) {
        return true
      }

      visited.put(current, ())
      current = current.next
    }

    false
  }

  def hasCycleNoAllocation(head: ListNode): Boolean = {
    // Instead of creating a set we can use Floyd's algorithm which is
    // a sneaky compsci trick exploiting the properties of graphs to
    // detect cycles.
    //
    // The idea is to have two pointers, fast and slow. Both start at the
    // head but fast moves two nodes at a time while slow just moves one
    // node at a time.
    //
    // If there isn't a cycle the pointers will _never_ intersect since `slow`
    // can never catch `fast`.
    //
    // If there is a cycle the pointers will _always_ intersect because the distance
    // between the two pointers will gradually shrink each cycle until both pointers
    // are intersecting!
    var slow = head
    var fast = head

    while (slow != null && fast != null && fast.next != null) {
      slow = slow.next
      fast = fast.next.next

      if(slow eq fast) {
        return true
      }
    }

    false
  }
}
