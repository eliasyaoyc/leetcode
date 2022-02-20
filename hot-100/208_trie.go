package hot_100

type Trie struct {
	child [26]*Trie
	end   bool
}

func Constructor2() Trie {
	return Trie{}
}

func (this *Trie) Insert(word string) {
	node := this
	for _, ch := range word {
		ch -= 'a'
		if node.child[ch] == nil {
			node.child[ch] = &Trie{}
		}
		node = node.child[ch]
	}
	node.end = true
}

func (this *Trie) SearchPrefix(prefix string) *Trie {
	node := this
	for _, ch := range prefix {
		ch -= 'a'
		if node.child[ch] == nil {
			return nil
		}
		node = node.child[ch]
	}
	return node
}

func (this *Trie) Search(word string) bool {
	node := this.SearchPrefix(word)
	return node != nil && node.end
}

func (this *Trie) StartsWith(prefix string) bool {
	return this.SearchPrefix(prefix) != nil
}
