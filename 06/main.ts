import { readFileSync } from "fs";

const data = readFileSync("input.txt", "utf-8");

function calc(bound: number) {
  for (let i = bound; i < data.length; i++) {
    if (Array.from(new Set(data.slice(i - bound, i + 1))).length > bound) return i + 1;
  }
}

console.log(calc(3));
console.log(calc(13));
