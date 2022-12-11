let divisor: number;

class Monkey {
  items: number[] = [];
  operationVal?: number;
  operator: string;
  testVal: number;
  inspected = 0;
  partnerMonkeys: Monkey[] = [];
  operation = (item: number) => {
    this.inspected++;
    if (this.operator === "+" && divisor) return Math.floor((item + (this.operationVal || item)) % divisor);
    if (this.operator === "*" && divisor) return Math.floor((item * (this.operationVal || item)) % divisor);
    if (this.operator === "+") return Math.floor((item + (this.operationVal || item)) / 3);
    if (this.operator === "*") return Math.floor((item * (this.operationVal || item)) / 3);
  };
  test = (item: number, monkeys: Monkey[]) => {
    if (item % this.testVal === 0) monkeys[0].items.push(item);
    else monkeys[1].items.push(item);
    this.items = [];
  };
  constructor(items: number[], testVal: number, operator: string, operationVal?: number) {
    this.items = items;
    this.operationVal = operationVal;
    this.operator = operator;
    this.testVal = testVal;
  }
}

const solve = (iterations: number, setDivisor?: boolean) => {
  // no time for input parsing today
  const monkey0 = new Monkey([54, 82, 90, 88, 86, 54], 11, "*", 7);
  const monkey1 = new Monkey([91, 65], 5, "*", 13);
  const monkey2 = new Monkey([62, 54, 57, 92, 83, 63, 63], 7, "+", 1);
  const monkey3 = new Monkey([67, 72, 68], 2, "*");
  const monkey4 = new Monkey([68, 89, 90, 86, 84, 57, 72, 84], 17, "+", 7);
  const monkey5 = new Monkey([79, 83, 64, 58], 13, "+", 6);
  const monkey6 = new Monkey([96, 72, 89, 70, 88], 3, "+", 4);
  const monkey7 = new Monkey([79], 19, "+", 8);
  monkey0.partnerMonkeys = [monkey2, monkey6];
  monkey1.partnerMonkeys = [monkey7, monkey4];
  monkey2.partnerMonkeys = [monkey1, monkey7];
  monkey3.partnerMonkeys = [monkey0, monkey6];
  monkey4.partnerMonkeys = [monkey3, monkey5];
  monkey5.partnerMonkeys = [monkey3, monkey0];
  monkey6.partnerMonkeys = [monkey1, monkey2];
  monkey7.partnerMonkeys = [monkey4, monkey5];

  const monkeys = [monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7];

  divisor = setDivisor ? monkeys.map((monkey) => monkey.testVal).reduce((a, b) => a * b) : 0;

  for (let i = 0; i < iterations; i++) {
    for (const monkey of monkeys) {
      monkey.items.forEach((item) => {
        monkey.test(monkey.operation(item)!, monkey.partnerMonkeys);
      });
    }
  }
  const sortedMonkeys = monkeys.sort((a, b) => b.inspected - a.inspected);
  return sortedMonkeys[0].inspected * sortedMonkeys[1].inspected;
};

console.log(solve(20));
console.log(solve(10000, true));
