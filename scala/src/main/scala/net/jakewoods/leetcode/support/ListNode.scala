package net.jakewoods.leetcode.support

import java.util.Objects
import org.scalacheck.Arbitrary

/** Definition for single-linked list in leetcode
  *
  *  Only `x` and `next` exist in the leetcode version.
  *  All other methods are to make testing and debugging easier.
  */
final class ListNode(var _x: Int = 0) {
  var next: ListNode = null
  var x: Int = _x

  /** Copy constructor
    *
    * Performs a deep clone on the list
    */
  def this(other: ListNode) = {
    this(other.x)
    if (other.next != null) {
      this.next = new ListNode(other.next)
    }
  }

  def fold[A](acc: A)(f: (A, Int) => A): A = {
    var result = acc
    var current = this
    while (current != null) {
      result = f(result, current.x)
      current = current.next
    }
    result
  }

  def toList: List[Int] = {
    fold(List.empty[Int]) { case (acc, value) => acc.appended(value) }
  }

  def length: Int = {
    fold(0) { case (acc, _) => acc + 1 }
  }

  def deepClone: ListNode = {
    new ListNode(this)
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
    toList.mkString("=>")
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

  implicit lazy val arbitraryListNode: Arbitrary[ListNode] = Arbitrary(for {
    head <- Arbitrary.arbitrary[Int]
    tail <- Arbitrary.arbitrary[List[Int]]
    list = ListNode.listOf(head, tail : _*)
  } yield list)
}
