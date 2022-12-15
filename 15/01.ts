import { readFileSync } from "fs";

const input = readFileSync("input.txt", "utf-8")
  .replaceAll("Sensor at x=", "")
  .replaceAll(", y=", ",")
  .replaceAll(": closest beacon is at x=", "!")
  .split("\n")
  .map((line) => line.split("!"))
  .map((line) => line.map((line) => line.split(",").map((num) => parseInt(num))));

const CHECK = 2000000;
const checkRow = new Set();

for (const [sensor, beacon] of input) {
  let [sensorX, sensorY] = sensor;
  let [beaconX, beaconY] = beacon;
  const xdistance = Math.abs(sensorX - beaconX);
  const ydistance = Math.abs(sensorY - beaconY);
  const distance = xdistance + ydistance;
  expendSensor(sensor, distance);
}

function expendSensor(sensor: number[], distance: number) {
  const sensorX = sensor[0];
  const sensorY = sensor[1];
  if (sensorY < CHECK) {
    const val = distance - (CHECK - sensorY);
    for (let i = 0; i <= val; i++) {
      checkRow.add(sensorX + i);
      checkRow.add(sensorX - i);
    }
  }
  if (sensorY > CHECK) {
    const val = distance - (sensorY - CHECK);
    for (let i = 0; i <= val; i++) {
      checkRow.add(sensorX + i);
      checkRow.add(sensorX - i);
    }
  }
}

// - 1 because there is a beacon in that row
console.log(checkRow.size - 1);
