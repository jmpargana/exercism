export class List {
  constructor(list) {
    this.list = list || [];
  }

  append(other) {
    return new List([...this.list, ...other.list])
  }

  concat(listOfLists) {
    listOfLists.forEach(list => this.list.append(list)) 
    return new List(this.list)
  }

  filter(predicate) {
    this.list.forEach((elem, index) => {
      if (!predicate(elem)) {
        this.list.splice(index, 1)
      }
    })
    return new List(this.list)
  }

  map(func) {
    this.list.forEach(elem => func(elem));
    return new List(this.list)
  }

  length() {
    let len = 0;
    while (this.list[len]) len++
    return len;
  }

  foldl(func, acc) {
    this.list.forEach(elem => func(acc, elem))
    return acc
  }

  foldr(func, acc) {
    for (var i=this.length()-1; i>=0; i--) {
      acc = func(acc, this.list[i])
    }
    return acc
  }

  reverse() {
    let result = [];
    for (var i=this.length()-1; i>=0; i--) {
      result.push(this.list[i])
    }
    return new List(result)
  }
}
