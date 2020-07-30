package net.jakewoods.leetcode.support

import java.util.Objects

// Definition for single-linked list in leetcode
final class ListNode(var _x: Int = 0) {
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

  override def equals(other: Any): Boolean = {
    other match {
      case that: ListNode => {
        x == that.x && Objects.equals(next, that.next)
      }

      case _ => false
    }
  }

  override def hashCode(): Int = Objects.hash(x, next)

  override def toString(): String = {
    toList.mkString("->")
  }
}

object ListNode {
  def listOf(value: Int, values: Int*): ListNode = {
    val input = value :: values.toList
    ListNode.nodes(input : _*).head
  }

  def nodes(values: Int*): List[ListNode] = {
    val nodes = values.map(new ListNode(_))
    nodes.sliding(2).foreach {
      case Seq(a: ListNode, b: ListNode) => a.next = b
      case _ => {}
    }
    nodes.toList
  }
}
