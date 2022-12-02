import { readFileSync } from "fs";

const file = readFileSync("input.txt", "utf-8");
const rounds = file.split("\n");

let sum = 0;
for (const round of rounds) {
  //   sum += gameResult(round[0], round[2]) + ownScore(round[2])!;
  const ownMove = calculateOwnMove(round[0], round[2]);
  sum += gameResult(round[0], ownMove!) + ownScore(ownMove!)!;
}

console.log(sum);

function gameResult(opp: string, me: string) {
  if (opp === "A" && me === "X") return 3;
  if (opp === "B" && me === "Y") return 3;
  if (opp === "C" && me === "Z") return 3;
  if (opp === "A" && me === "Y") return 6;
  if (opp === "B" && me === "Z") return 6;
  if (opp === "C" && me === "X") return 6;
  return 0;
}

function ownScore(me: string) {
  if (me === "X") return 1;
  if (me === "Y") return 2;
  if (me === "Z") return 3;
}

function calculateOwnMove(opp: string, result: string) {
  if (result === "X") {
    if (opp === "A") return "Z";
    if (opp === "B") return "X";
    if (opp === "C") return "Y";
  }
  if (result === "Y") {
    if (opp === "A") return "X";
    if (opp === "B") return "Y";
    if (opp === "C") return "Z";
  }
  if (result === "Z") {
    if (opp === "A") return "Y";
    if (opp === "B") return "Z";
    if (opp === "C") return "X";
  }
}
