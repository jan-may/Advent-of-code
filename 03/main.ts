import { readFileSync } from "fs";
import { connected } from "process";

const file = readFileSync("input.txt", "utf-8");
const backpacks = file.split("\n");

let doubleItems = "";

for (const bp of backpacks) {
  const comp1 = bp.slice(0, bp.length / 2);
  const comp2 = bp.slice(bp.length / 2, bp.length);
  for (const char of comp1) {
    if (comp2.includes(char)) {
      doubleItems += char;
      break;
    }
  }
}

let tribbleItems = "";
const size = 3;
for (let i = 0; i < backpacks.length; i += size) {
  const chunk = backpacks.slice(i, i + size);
  console.log(chunk[0]);
  for (const char of chunk[0]) {
    if (chunk[1].includes(char) && chunk[2].includes(char)) {
      tribbleItems += char;
      break;
    }
  }
}

let result1 = 0;
let result2 = 0;
for (let i = 0; i < doubleItems.length; i++) {
  result1 += checkCharValue(doubleItems[i]);
}

for (let i = 0; i < tribbleItems.length; i++) {
  result2 += checkCharValue(tribbleItems[i]);
}

function checkCharValue(char: string) {
  if (char == char.toLowerCase()) return char.charCodeAt(0) - 96;
  return char.charCodeAt(0) - 38;
}

console.log(result1, result2);
