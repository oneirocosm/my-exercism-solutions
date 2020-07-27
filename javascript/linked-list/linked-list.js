export class LinkedList {
  constructor() {
    this.head = undefined;
    this.tail = undefined;
  }

  push(elem) {
    if (this.tail == undefined) {
      const new_node = new Node(elem);
      this.head = new_node;
      this.tail = new_node;
    } else {
      const old_tail = this.tail;
      const new_tail = new Node(elem);
      old_tail.next = new_tail;
      new_tail.prev = old_tail;
      this.tail = new_tail;
    }
  }

  pop() {
    const old_tail = this.tail;
    const new_tail = old_tail.prev;
    this.tail = new_tail;
    if (new_tail == undefined) {
      this.head = undefined;
    }
    return old_tail.elem;
  }

  shift() {
    const old_head = this.head;
    const new_head = old_head.next;
    this.head = new_head;
    if (new_head == undefined) {
      this.tail = undefined;
    }
    return old_head.elem;
  }

  unshift(elem) {
    if (this.head == undefined) {
      const new_node = new Node(elem);
      this.head = new_node;
      this.tail = new_node;
    } else {
      const old_head = this.head;
      const new_head = new Node(elem);
      old_head.prev = new_head;
      new_head.next = old_head;
      this.head = new_head;
    }
  }

  delete(elem) {
    let node = this.head;
    while (node != undefined) {
      if (node.elem == elem) {
        const prev = node.prev;
        const next = node.next;
        if (next != undefined) {
          next.prev = prev;
        }
        if (prev != undefined) {
          prev.next = next;
        }
        break;
      }
      node = node.next;
    }
    if (node === this.head) {
      this.head = this.head.next;
    }
    if (node === this.tail) {
      this.tail = this.tail.prev;
    }
  }

  count() {
    let count = 0;
    let node = this.head;
    while (node != undefined) {
      node = node.next;
      ++count;
    }
    return count;
  }
}

class Node {
  constructor(elem) {
    this.elem = elem;
    this.next = undefined;
    this.prev = undefined;
  }
}
