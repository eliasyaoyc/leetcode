package hot_100

type Node struct {
	key, value int
	prev, next *Node
}

type LRUCache struct {
	size       int
	cap        int
	cache      map[int]*Node // 这个map中的key 是 node 的key
	head, tail *Node
}

func Constructor(capacity int) LRUCache {
	l := LRUCache{
		size:  0,
		cap:   capacity,
		cache: map[int]*Node{},
		head: &Node{
			key:   0,
			value: 0,
		},
		tail: &Node{
			key:   0,
			value: 0,
		},
	}
	l.head.next = l.tail
	l.tail.prev = l.head
	return l
}

func (this *LRUCache) Get(key int) int {
	if _, ok := this.cache[key]; !ok {
		return -1
	}
	node := this.cache[key]
	this.moveToHead(node)
	return node.value
}

func (this *LRUCache) Put(key int, value int) {
	if _, ok := this.cache[key]; !ok {
		node := &Node{key: key, value: value}
		this.cache[key] = node
		this.addToHead(node)
		this.size++
		if this.size > this.cap {
			removed := this.removeTail()
			delete(this.cache, removed.key)
			this.size--
		}
	} else {
		node := this.cache[key]
		node.value = value
		this.moveToHead(node)
	}
}

func (this *LRUCache) addToHead(node *Node) {
	node.prev = this.head
	node.next = this.head.next
	this.head.next.prev = node
	this.head.next = node
}

func (this *LRUCache) removeNode(node *Node) {
	node.prev.next = node.next
	node.next.prev = node.prev
}

func (this *LRUCache) moveToHead(node *Node) {
	this.removeNode(node)
	this.addToHead(node)
}

func (this *LRUCache) removeTail() *Node {
	node := this.tail.prev
	this.removeNode(node)
	return node
}
