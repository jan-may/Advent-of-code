"use strict";
exports.__esModule = true;
// read a txt file
var fs_1 = require("fs");
var text = (0, fs_1.readFileSync)("input.txt", "utf-8");
// split text into array of calories
var temp = text.split("\n").map(Number);
var max = 0;
var maxTemp = 0;
var calories = [];
var tempCaloriesArr = [];
for (var i = 0; i < temp.length; i++) {
    if (temp[i] !== 0) {
        tempCaloriesArr.push(temp[i]);
    }
    else {
        calories.push(tempCaloriesArr);
        tempCaloriesArr = [];
    }
}
var totalCalories = calories.map(function (calorie) { return calorie.reduce(function (a, b) { return a + b; }, 0); }).sort(function (a, b) { return a - b; });
console.log(totalCalories.at(-1) + totalCalories.at(-2) + totalCalories.at(-3));
