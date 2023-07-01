import { readFileSync } from "fs";

const file1 = readFileSync("input1.txt", "utf8");
const file2 = readFileSync("input2.txt", "utf8");

const crates = file1.replaceAll("    ", " [X]").replaceAll("[", "").replaceAll("]", "").replaceAll(" ", "").split("\n");
const moves = file2.replaceAll("move ", "").replaceAll(" from ", "?").replaceAll(" to ", "?").split("\n");
const stack: any = [[], [], [], [], [], [], [], [], []];

// formating the stack
for (let i = 0; i < crates.length; i++) {
  for (let j = 0; j <= crates.length; j++) {
    if (crates[i][j] != "X") {
      stack[j].unshift(crates[i][j]);
    }
  }
}

// Part 1
function moveCrates(num: number, start: number, dest: number) {
  for (let i = 0; i < num; i++) {
    const crate = stack[start - 1].pop();
    stack[dest - 1].push(crate);
  }
}

// Part 2
function moveCrates2(num: number, start: number, dest: number) {
  if (num === 1) {
    stack[dest - 1].push(stack[start - 1].pop());
  } else {
    const subArr = stack[start - 1].slice(stack[start - 1].length - num, stack[start - 1].length);
    for (let i = 0; i < subArr.length; i++) {
      stack[start - 1].pop();
      stack[dest - 1].push(subArr[i]);
    }
  }
}

for (const move of moves) {
  const formatedMove = move.split("?").map(Number);
  //   moveCrates1(formatedMove[0], formatedMove[1], formatedMove[2]);
  moveCrates2(formatedMove[0], formatedMove[1], formatedMove[2]);
}

console.log(stack);
