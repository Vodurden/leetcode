package net.jakewoods.leetcode.support

import java.util.Objects
import org.scalacheck.Arbitrary

/** Definition for single-linked list in leetcode
  *
  *  Only `x` and `next` exist in the leetcode version.
  *  All other methods are to make testing and debugging easier.
  *
  * This version is generic to make some of the helper function easier
  * to implement. For a full mock of the leetcode version see `ListNode`
  */
final class ListNode(var _x: Int = 0) {
  var next: ListNode = null
  var x: Int = _x

  type Index = Int

  /** `foldNodes` folds over a list, detecting cycles and counting the index of the current node.
    *
    * Cycle detection only applies from `this` to the end of the list. If this node is part of a
    * longer list and the cycle points to an earlier part of the list some values will repeat until
    * `this` is traversed again.
    *
    * This function is used to derive most other methods, even when it would be more efficient to
    * specialize them because all methods on this class are used to test relatively small sample
    * sizes
    */
  def foldNodes[A](
    acc: A,
    onCycle: (A, Index, ListNode) => A
  )(f: (A, Index, ListNode) => A): A = {
    // We need to check for cycles when we fold since
    // some problems create cycles and we don't want
    // infinite loops in our tests.
    val visited = new java.util.IdentityHashMap[ListNode, Unit]()

    var result = acc
    var current = this
    var currentIndex = 0
    while (current != null && !visited.containsKey(current)) {
      result = f(result, currentIndex, current)

      visited.put(current, ())
      current = current.next
      currentIndex += 1
    }

    // If we're here and current isn't null then it must be
    // the node we've cycled to
    if (current != null) {
      var cycleIndex = 0
      var cycleIndexCurrent = this
      while (!(cycleIndexCurrent eq current)) {
        cycleIndex += 1
        cycleIndexCurrent = cycleIndexCurrent.next
      }

      result = onCycle(result, cycleIndex, current)
    }

    result
  }

  def foldNodes[A](acc: A)(f: (A, Index, ListNode) => A): A = {
    foldNodes(acc, (x: A, _, _) => x)(f)
  }

  /** Find the node and index of the list item that matches `predicate`
    *
    * @param predicate a function that returns true if this is the node we want to find
    * @return the index and node that matched the predicate, or None
    */
  def findWhereWithIndex(
    predicate: (Index, ListNode) => Boolean
  ): Option[(Index, ListNode)] = {
    foldNodes(None : Option[(Index, ListNode)]) { case (result, index, node) =>
      if(predicate(index, node) && result == None) {
        Some((index, node))
      } else {
        result
      }
    }
  }

  def findWhere(f: ListNode => Boolean): Option[ListNode] = {
    findWhereWithIndex { case (_, node) => f(node) }.map(_._2)
  }

  def toList: List[Int] = {
    foldNodes(List.empty[Int]) { case (acc, _, node) => acc.appended(node.x) }
  }

  def length: Int = {
    foldNodes(0) { case (acc, _, _) => acc + 1 }
  }

  /**
    * Creates a new linked list with the same values and `next` structure as `this`
    *
    * If `this` contains a cycle then the cloned list will also contain a cycle.
    *
    * @return the head of the cloned list
    */
  def deepClone: ListNode = {
    var preHead = new ListNode(-1)

    val onCycle = (resultTail: ListNode, index: Index, _: ListNode) => {
      // We do `index + 1` because `preHead` is not part
      // of the result list
      preHead = preHead.withCycle(index + 1)
      resultTail
    }

    foldNodes(preHead, onCycle) { case (resultTail, _, node) =>
      resultTail.next = new ListNode(node.x)
      resultTail.next
    }

    preHead.next
  }

  /** Get the node at some index
    */
  def at(targetIndex: Int): Option[ListNode] = {
    findWhereWithIndex { case (index, _) => targetIndex == index }.map(_._2)
  }

  def last: Option[ListNode] = {
    this.findWhere(node => node.next == null)
  }

  /** Make the end of this linked list cycle back to a previous node.
    *
    * @param cycleTargetIndex: The index of the node to cycle back to
    */
  def withCycle(cycleTargetIndex: Int): ListNode = {
    val target = this.at(cycleTargetIndex).orNull
    this.last.map(last => last.next = target)

    this
  }

  override def equals(other: Any): Boolean = {
    other match {
      case that: ListNode => {
        var current = this
        that.foldNodes(true) { case (acc, _, node) =>
          val result = acc && current.x == node.x
          current = current.next
          result
        }
      }

      case _ => false
    }
  }

  override def hashCode(): Int = Objects.hash(x, next)

  override def toString(): String = {
    val onCycle = (acc: String, index: Index, _: ListNode) =>
      s"${acc}=>(Cycle to $index)"

    this.foldNodes("", onCycle) { case (acc, _, node) =>
      s"${acc}=>${node.x}"
    }.drop(2)
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
