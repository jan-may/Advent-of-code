import { readFileSync } from "fs";

type Pos = [number, number];

const data = readFileSync("input.txt", "utf-8")
  .split("\n")
  .map((line) => line.split(""));

let start: Pos[];
let dest: Pos;

for (let i = 0; i < data.length; i++) {
  for (let j = 0; j < data[i].length; j++) {
    if (data[i][j] === "S") start = [[i, j]];
    if (data[i][j] === "E") dest = [i, j];
  }
}

const grid = data.map((line) => line.map((char) => char.replaceAll("S", "a").replaceAll("E", "z").charCodeAt(0) - 97));

const start2 = grid.map((row, i) => row.filter((char, j) => char === 0 && [i, j])) as Pos[];

const maxWidth = grid.length;
const maxHeight = grid[0].length;

function getGoodNeighbours(pos: Pos) {
  const [x, y] = pos;
  const height = grid[x][y];
  const positions: Pos[] = [];
  if (x + 1 < maxWidth && grid[x + 1][y] - height <= 1) positions.push([x + 1, y]);
  if (x - 1 >= 0 && grid[x - 1][y] - height <= 1) positions.push([x - 1, y]);
  if (y - 1 >= 0 && grid[x][y - 1] - height <= 1) positions.push([x, y - 1]);
  if (y + 1 < maxHeight && grid[x][y + 1] - height <= 1) positions.push([x, y + 1]);
  return positions;
}

function bfs(start: Pos[], dest: Pos) {
  for (const startPoint of start) {
    const queue: Pos[] = [startPoint];
    const visited = new Set<string>();
    const prev = new Map<string, Pos>();
    const dist = new Map<string, number>();
    const key = (pos: Pos) => pos.join(",");

    dist.set(key(startPoint), 0);

    while (queue.length) {
      const pos = queue.shift()!;
      const neighbors = getGoodNeighbours(pos);
      for (const neighbor of neighbors) {
        if (visited.has(key(neighbor))) continue;
        visited.add(key(neighbor));
        dist.set(key(neighbor), dist.get(key(pos))! + 1);
        prev.set(key(neighbor), pos);
        queue.push(neighbor);
      }
    }
    return dist.get(key(dest))!;
  }
}

console.log(bfs(start!, dest!));
console.log(bfs(start2, dest!));
