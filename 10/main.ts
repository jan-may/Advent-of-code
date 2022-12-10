import { readFileSync } from "fs";

const input = readFileSync("input.txt", "utf8");
const instructions = input.split("\n");

let cycle = 0;
let x = 1;
let signalStrength: number[] = [];
const screen: string[] = new Array(6).fill("");

function checkPushable(cycle: number) {
  if (cycle === 20 || (cycle - 20) % 40 === 0) signalStrength.push(x * cycle);
}

function incrementCycle() {
  cycle++;
  checkPushable(cycle);
  drawPixel();
}

function drawPixel() {
  let row = Math.floor(((cycle - 1) / 40) % 6);
  let index = screen[row].length;
  screen[row] += index >= x - 1 && index <= x + 1 ? "#" : ".";
}

for (const instruction of instructions) {
  incrementCycle();
  if (instruction.startsWith("noop")) {
    continue;
  }
  const [, val] = instruction.split(" ");
  incrementCycle();
  x += +val;
}

// Part 1
console.log(signalStrength.reduce((a, b) => a + b, 0));

// Part 2
console.log(screen);
