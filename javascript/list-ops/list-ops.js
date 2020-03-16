export class List {
  constructor(values) {
    this.values = values || [] 
  }

  append(other) {
    return new List([...this.values, ...other.values])
  }

  concat(listOfLists) {
    let newList = this;
    listOfLists.values.map(list => newList = newList.append(list))
    return newList
  }

  filter(func) {
    let result = []
    for (var i = 0; this.values.length; i++) 
      if (func(this.values[i])) result.push(this.values[i])
   
    return new List(result)
  }

  map(func) {
    let result = []
    for (var i = 0; i < this.values.length; i++) result.push(func(this.values[i]))
    return new List(result)
  }

  length() {
    let len = 0;
    for (var i = 0; i < this.values.length; i++) len++
    return len;
  }

  foldl(func, start) {
    let acc = start
    for (let i = 0; i < this.values.length; i++) 
      acc = func(acc, this.values[i])

    return acc
  }

  foldr(func, start) {
    let acc = start
    for (let i = this.values.length - 1; i >= 0; i--)
      acc = func(acc, this.values[i])

    return acc
  }

  reverse() {
    let result = []
    for (let i = this.values.length - 1; i >= 0; i--) 
      result.push(this.values[i])

    return new List(result)
  }
}
