// Part 2
import { readFileSync } from "fs";

const input = readFileSync("input.txt", "utf-8");
const treeMap = input.split("\n").map((line) => line.split("").map((val) => +val));
const maxWidth = treeMap[0].length;
const maxHeight = treeMap.length;
let maxScore = 0;

function horizontalScore(i: number, j: number, direction: "left" | "right"): number {
  if (direction === "left" && j === 0) return 0;
  if (direction === "right" && j === maxWidth) return 0;
  let score = 0;
  let curr = treeMap[i][j];
  direction === "left" ? j-- : j++;
  while (direction === "left" ? j >= 0 : j < maxWidth) {
    score++;
    if (curr <= treeMap[i][j]) break;
    direction === "left" ? j-- : j++;
  }
  return score;
}

function verticalScore(i: number, j: number, direction: "top" | "bottom"): number {
  if (direction === "top" && i === 0) return 0;
  if (direction === "bottom" && i === maxHeight) return 0;
  let score = 0;
  let curr = treeMap[i][j];
  direction === "top" ? i-- : i++;
  while (direction === "top" ? i >= 0 : i < maxHeight) {
    score++;
    if (curr <= treeMap[i][j]) break;
    direction === "top" ? i-- : i++;
  }
  return score;
}

function calcTotalScore(i: number, j: number) {
  const score =
    horizontalScore(i, j, "left") *
    horizontalScore(i, j, "right") *
    verticalScore(i, j, "top") *
    verticalScore(i, j, "bottom");
  return score;
}

for (let i = 0; i < maxWidth; i++) {
  for (let j = 0; j < maxHeight; j++) {
    const score = calcTotalScore(i, j);
    if (score > maxScore) maxScore = score;
  }
}

console.log(maxScore);
