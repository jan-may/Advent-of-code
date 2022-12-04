import { readFileSync } from "fs";

const file = readFileSync("input.txt", "utf-8");
const data = file.replaceAll(",", "-").split("\n");

function isFitting(min: number, max: number, value: number) {
  return value >= min && value <= max;
}

function isOverlapping(min1: number, max1: number, min2: number, max2: number) {
  return (
    isFitting(min1, max1, min2) ||
    isFitting(min1, max1, max2) ||
    isFitting(min2, max2, min1) ||
    isFitting(min2, max2, max1)
  );
}

// Part 1
let count1 = 0;
for (const line of data) {
  const pairs = line.split("-").map(Number);
  if (isFitting(pairs[0], pairs[1], pairs[2]) && isFitting(pairs[0], pairs[1], pairs[3])) {
    count1++;
  } else if (isFitting(pairs[2], pairs[3], pairs[0]) && isFitting(pairs[2], pairs[3], pairs[1])) {
    count1++;
  }
}
console.log(count1);

// Part 2
let count2 = 0;
for (const line of data) {
  const pairs = line.split("-").map(Number);
  if (isOverlapping(pairs[0], pairs[1], pairs[2], pairs[3])) {
    count2++;
  }
}
console.log(count2);
