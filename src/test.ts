enum TSEnum {
    A = 'a',
    B = 'b',
    C = 'c'
}

console.log(TSEnum.A, TSEnum.B, TSEnum.C);


const a: number[] = [];
const b = a;

b.push(1);

console.log(a);
