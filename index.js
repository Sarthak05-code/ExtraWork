const array = async() => {
  const Data = await fetch('https://dummyjson.com/products')
  const res = await Data.json()
  const products = res.products

  for (let i = 0; i < products.length; ++i) {
    console.log(products[i].title)
  }
}

array()