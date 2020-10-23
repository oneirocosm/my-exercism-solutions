export class LinkedList {
  constructor() {
    this.head = null;
    this.tail = null;
  }

  push(elem) {
    const new_tail = new Node(elem);
    if (this.tail === null) {
      this.head = new_tail;
      this.tail = new_tail;
    } else {
      const old_tail = this.tail;
      old_tail.next = new_tail;
      new_tail.prev = old_tail;
      this.tail = new_tail;
    }
  }

  pop() {
    const old_tail = this.tail;
    const new_tail = old_tail.prev;
    this.tail = new_tail;
    if (new_tail === null) {
      this.head = null;
    }
    return old_tail.elem;
  }

  shift() {
    const old_head = this.head;
    const new_head = old_head.next;
    this.head = new_head;
    if (new_head === null) {
      this.tail = null;
    }
    return old_head.elem;
  }

  unshift(elem) {
    const new_head = new Node(elem);
    if (this.head === null) {
      this.head = new_head;
      this.tail = new_head;
    } else {
      const old_head = this.head;
      old_head.prev = new_head;
      new_head.next = old_head;
      this.head = new_head;
    }
  }

  delete(elem) {
    let node = this.head;
    while (node !== null) {
      if (node.elem == elem) {
        const prev = node.prev;
        const next = node.next;
        if (next !== null) {
          next.prev = prev;
        }
        if (prev !== null) {
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
    while (node !== null) {
      node = node.next;
      ++count;
    }
    return count;
  }
}

class Node {
  constructor(elem) {
    this.elem = elem;
    this.next = null;
    this.prev = null;
  }
}
