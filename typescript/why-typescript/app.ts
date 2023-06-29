// understanding dynamic types
let box;
console.log(typeof(box)); // undefined

box = "Hello";
console.log(typeof(box)); // string

box = 100;
console.log(typeof(box)); // number

let num = 2020;

let box2;
console.log(typeof(box)); // undefined

box2 = "Hello";
console.log(typeof(box2)); // string

box2 = 100;
console.log(typeof(box2)); // number

// Problems with dynamic types
function getProduct(id) {
    return {
        id: id,
        name: `Awesome Gadget ${id}`,
        price: 99.5
    }
}

const product = getProduct(1);
console.log(`The product ${product.name} costs $${product.price}`);

const showProduct = (name, price) => {
    console.log(`The product ${name} costs ${price}$.`);
};

const product2 = getProduct(1);
showProduct(product2.price, product2.name);

// How Typescript solves the problems of dynamic types

interface Product {
    id: number,
    name: String,
    price: number;
};

function getProduct2(id) : Product {
    return {
        id: id,
        name: `Awesome Gadget ${id}`,
        price: 99.5
    }
}

const product3 = getProduct2(1);
console.log(`The product ${product3.name} costs $${product.price}`);

const showProduct2 = (name: string, price: number) => {
    console.log(`The product ${name} costs ${price}$.`);
}

