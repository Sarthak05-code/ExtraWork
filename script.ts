const object = {
  Array: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
  value: function (x: number, y: number): number {
    return x * y + 10
  }
}
console.log(`the number are : ${object.Array[2]} and ${object.Array[8]}`)
console.log(object.value(object.Array[2], object.Array[8]))