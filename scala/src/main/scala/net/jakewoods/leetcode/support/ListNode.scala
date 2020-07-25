package net.jakewoods.leetcode.support

// Definition for single-linked list in leetcode
class ListNode(var _x: Int = 0) {
  var next: ListNode = null
  var x: Int = _x

  def toList: List[Int] = {
    var result = List[Int]()
    var current = this
    while (current != null) {
      result = result.appended(current.x)
      current = current.next
    }
    result
  }
}

object ListNode {
  def nodes(values: Int*): List[ListNode] = {
    val nodes = values.map(new ListNode(_))
    nodes.sliding(2).foreach {
      case Seq(a: ListNode, b: ListNode) => a.next = b
      case _ => {}
    }
    nodes.toList
  }
}
