function factorial(n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}

function hesabla() {
    let cemi = 0;
    for (let i = 1; i <= 5000000; i++) {
        cemi += factorial(15) + i;
    }
    return cemi;
}

const baslat = Date.now();
const netice = hesabla();
const bitir = Date.now();

console.log(`Cəmi: ${netice}`);
console.log(`Vaxt: ${bitir - baslat} ms`);
