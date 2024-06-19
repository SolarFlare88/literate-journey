const wasm = require('wasm_function');

async function run() {
    // const result = wasm.add('5', '3');
    // console.log(`Result of 5 + 3 is: ${result}`);

    // const fibResult = wasm.fibonacci(10);
    // console.log('fibonacci 10: ', fibResult);

    const a = wasm.calculate_product();
    console.log(a);

    console.time('js');
    let product = 1;
    for (let i = 1; i<= 1000; i++) {
        product *= i;
    }
    console.log(product);
    console.timeEnd('js');
}

run();
