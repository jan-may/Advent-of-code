import { readFileSync } from "fs";

const input = readFileSync("input.txt", "utf-8")
  .replaceAll("Sensor at x=", "")
  .replaceAll(", y=", ",")
  .replaceAll(": closest beacon is at x=", "!")
  .split("\n")
  .map((line) => line.split("!"))
  .map((line) => line.map((line) => line.split(",").map((num) => parseInt(num))));

const MAX = 4000000;

for (let i = 0; i < MAX; i++) {
  let slices: number[][] = [];
  let distinctSlices: number[][] = [];

  for (const [sensor, beacon] of input) {
    const distance = Math.abs(sensor[0] - beacon[0]) + Math.abs(sensor[1] - beacon[1]);
    let width = distance - Math.abs(sensor[1] - i);
    if (width >= 0) slices.push([sensor[0] - width, sensor[0] + width]);
  }

  slices.sort((a, b) => a[0] - b[0]);

  let [min, max] = slices[0];
  for (let j = 1; j < slices.length; j++) {
    if (slices[j][0] <= max) {
      max = Math.max(max, slices[j][1]);
    } else {
      distinctSlices.push([min, max]);
      [min, max] = slices[j];
    }
  }
  if (min <= MAX && 0 <= max) distinctSlices.push([min, max]);
  if (distinctSlices.length > 1) {
    console.log((distinctSlices[0][1] + 1) * MAX + i);
    break;
  }
}
