import { readFileSync } from "fs";

type List = number[] | List[];
type Pair = [List, List];

function isNum(n: number | List): n is number {
  return typeof n === "number";
}

const input = readFileSync("input.txt", "utf-8");

const pairsArr: [List, List][] = input
  .split("\n\n")
  .map((pair) => pair.split("\n").map((arr) => JSON.parse(arr)) as Pair);

function compare(left: List, right: List): number {
  for (let [i, l] of left.entries()) {
    let val = 0;
    let r = right[i];
    if (!r) return 1;
    if (isNum(l) && isNum(r)) val = l - r;
    else val = compare(isNum(l) ? [l] : l, isNum(r) ? [r] : r);
    if (val !== 0) return val;
  }
  return left.length - right.length;
}

let result = [];

for (const [i, [leftArr, rightArr]] of pairsArr.entries()) {
  let score = compare(leftArr, rightArr);
  if (score < 0) result.push({ i, score });
}

const flatArr = pairsArr
  .flatMap((pair) => pair)
  .concat([[[2]], [[6]]])
  .sort(compare);
const indexOf2 = flatArr.findIndex((arr) => JSON.stringify(arr) === "[[2]]") + 1;
const idnexOf6 = flatArr.findIndex((arr) => JSON.stringify(arr) === "[[6]]") + 1;

console.log("Part 1: " + result.reduce((sum, { i }) => sum + i + 1, 0));
console.log("Part 2: " + indexOf2 * idnexOf6);
