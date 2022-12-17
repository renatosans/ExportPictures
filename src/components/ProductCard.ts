
export const ProductCard = (name: string, description: string, price: number) => {
return `
	<div class="card">
	<div class="card-img">
		<img src="https://i.ibb.co/HPxvK8w/nike-sneakers.png" alt="Nike" />
	</div>
	<div class="card-info">
		<h1 class="card-sneakers-name">${name}</h1>
		<p class="card-sneakers-description">${description}</p>
		<div class="card-small-info">
		<p class="product-price">${price}</p>
		</div>
	</div>
	</div>
`
}
