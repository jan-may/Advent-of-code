// read a txt file
import { readFileSync } from "fs";
const text = readFileSync("input.txt", "utf-8");

const temp = text.split("\n").map(Number);

let calories: number[][] = [];
let tempCaloriesArr: number[] = [];

for (let i = 0; i < temp.length; i++) {
  if (temp[i] !== 0) {
    tempCaloriesArr.push(temp[i]);
  } else {
    calories.push(tempCaloriesArr);
    tempCaloriesArr = [];
  }
}

const totalCalories = calories.map((calorie) => calorie.reduce((a, b) => a + b, 0)).sort((a, b) => a - b);

console.log(totalCalories.at(-1));
console.log(totalCalories.at(-1)! + totalCalories.at(-2)! + totalCalories.at(-3)!);
