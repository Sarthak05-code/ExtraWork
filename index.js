const fetchData = async () => {
    const Data = await fetch('https://dummyjson.com/products')
    const res = await Data.json()
    const products = res.products

    const tbody = document.querySelector('#dataFetcher tbody')
    for(let i = 0 ; i < products.length ; ++i) {
        const row = document.createElement('tr')
        row.innerHTML = `
            <td>${products[i].id}</td>
            <td>${products[i].title}</td>
            <td>${products[i].price}</td>

        `
        tbody.appendChild(row)
    }
}

fetchData()