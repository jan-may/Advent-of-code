// Part 1
import { readFileSync } from "fs";

const input = readFileSync("input.txt", "utf-8");
const treeMap = input.split("\n").map((line) => line.split("").map((val) => +val));

// Border values are always visible
let allwaysVisible = (treeMap.length + treeMap[0].length) * 2 - 4;
let visited = new Set();

for (let i = 1, len = treeMap.length - 1; i < len; i++) {
  let max = treeMap[i][0];
  for (let j = 1, len = treeMap[i].length - 1; j < len; j++) {
    max = check(i, j, max, "left");
  }
  max = treeMap[i][treeMap[i].length - 1];
  for (let j = treeMap[i].length - 2; j > 0; j--) {
    max = check(i, j, max, "right");
  }
}

for (let i = 1, len = treeMap[0].length - 1; i < len; i++) {
  let max = treeMap[0][i];
  for (let j = 1, len = treeMap.length - 1; j < len; j++) {
    max = check(i, j, max, "top");
  }
  max = treeMap[treeMap.length - 1][i];
  for (let j = treeMap.length - 2; j > 0; j--) {
    max = check(i, j, max, "bottom");
  }
}

function check(i: number, j: number, max: number, direction: "left" | "right" | "top" | "bottom") {
  let curr;
  direction === "left" || direction === "right" ? (curr = treeMap[i][j]) : (curr = treeMap[j][i]);
  if (curr > max) {
    direction === "left" || direction === "right" ? (max = curr) : (max = curr);
    direction === "left" || direction === "right" ? visited.add(` ${i} ${j}`) : visited.add(` ${j} ${i}`);
  }
  return max;
}

console.log(visited.size + allwaysVisible);
