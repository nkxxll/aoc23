from enum import Enum


class Direction(Enum):
    LEFT = 0
    RIGHT = 1

    @classmethod
    def from_str(cls, string: str):
        if string == "R":
            return Direction.RIGHT
        elif string == "L":
            return Direction.LEFT
        else:
            raise ValueError(f"Invalid direction {string}")


class SolveMap:
    def init(self):
        self._instrucitons: list[Direction] = []
        self._map = dict()

    def parse_input(self, input_file: str = "./inputs/input08.txt"):
        with open(input_file) as f:
            lines = f.read()

        instructions, map = lines.split("\n\n")
        self._map = self._parse_map_lines(
            list(filter(lambda x: x != "", map.split("\n")))
        )
        self._instrucitons = [
            (Direction.from_str(instructions[i : i + 1]))
            for i in range(0, len(instructions), 1)
        ]

    def _instruction_generator(self):
        count = 0
        mod = len(self._instrucitons)
        while True:
            yield self._instrucitons[count % mod]
            count += 1

    @staticmethod
    def _parse_map_lines(lines: list[str]):
        map = dict()
        for item in lines:
            parts = item.split("=")
            key = parts[0].strip()
            values = parts[1].strip().split(",")
            for i, val in enumerate(values):
                values[i] = val.replace("(", "").replace(")", "").strip()
            map[key] = values
        return map

    def _get_starts(self):
        starts = []
        for key in self._map.keys():
            if key[2] == "A":
                starts.append(key)
        return starts

    def _generate_start_gen(self, start: str):
        nextest = start
        dir_gen = self._instruction_generator()
        while True:
            nextest = self._map[nextest][next(dir_gen).value]
            yield nextest

    def _generate_start_gens(self, starts: list[str]):
        start_gens = []
        for start in starts:
            start_gens.append(self._generate_start_gen(start))
        print(start_gens)
        return start_gens

    def walk_like_a_ghost(self):
        # FIXME: this is right but to slow for numbers highter than 1 << 32 lol
        starts = self._get_starts()
        len_starts = len(starts)
        start_gens = self._generate_start_gens(starts)
        count = 0
        nexts = ["" for _ in range(len(starts))]
        while True:
            for idx, start in enumerate(start_gens):
                nexts[idx] = next(start)
            count += 1
            assert count < 1 << 20
            c = 0
            for n in nexts:
                c += 1 if n[2] == "Z" else 0
            if c == len_starts:
                break
                
        return count

    def walk_map(self):
        nextest = "AAA"
        end = "ZZZ"
        count = 0
        instruction = self._instruction_generator()
        while True:
            nextest = self._map[nextest][next(instruction).value]
            count += 1
            if nextest == end:
                break
        return count


def main():
    sol = SolveMap()
    sol.parse_input()
    print(sol.walk_like_a_ghost())


if __name__ == "__main__":
    main()
