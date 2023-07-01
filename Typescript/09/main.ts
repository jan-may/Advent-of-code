import { readFileSync } from "fs";

const input = readFileSync("input.txt", "utf8");

function calcUnique(input: string, n: number): number {
  const lines = readFileSync("input.txt", "utf8").split("\n");
  const knots = new Array(n).fill(null).map(() => ({ x: 0, y: 0 }));
  const result = new Set([`${knots[0].x},${knots[0].y}`]);

  for (const line of lines) {
    const [direction, val] = line.split(" ");
    const steps = +val;
    for (let i = 0; i < steps; i++) {
      const head = knots[knots.length - 1];
      Move(direction, head);
      for (let k = knots.length - 2; k >= 0; k--) {
        const { absX, absY, x, knot, y } = getValues(knots, k);
        if (absX > 1 || absY > 1) {
          if (x !== 0) knot.x += x / absX;
          if (y !== 0) knot.y += y / absY;
        }
      }
      result.add(`${knots[0].x},${knots[0].y}`);
    }
  }
  return result.size;
}

const getValues = (knots: { x: number; y: number }[], k: number) => {
  const parent = knots[k + 1];
  const knot = knots[k];
  const x = parent.x - knot.x;
  const absX = Math.abs(x);
  const y = parent.y - knot.y;
  const absY = Math.abs(y);
  return { absX, absY, x, knot, y };
};

const Move = (direction: string, head: { x: number; y: number }) => {
  if (direction === "R") head.x++;
  if (direction === "L") head.x--;
  if (direction === "U") head.y++;
  if (direction === "D") head.y--;
};

// Part 1
console.log(calcUnique(input, 2));

// Part 2
console.log(calcUnique(input, 10));
