from unittest import TestCase
from main import SolveMap, Direction

class TestMain(TestCase):
    def test_directions_generator(self):
        sol = SolveMap()
        sol.parse_input("./inputs/input08.txt")
        gen = sol._instruction_generator()
        self.assertEqual(next(gen), Direction.RIGHT)
        self.assertEqual(next(gen), Direction.LEFT)
        self.assertEqual(next(gen), Direction.RIGHT)
        self.assertEqual(next(gen), Direction.LEFT)
        self.assertEqual(next(gen), Direction.RIGHT)
        self.assertEqual(next(gen), Direction.LEFT)
        self.assertEqual(next(gen), Direction.RIGHT)
        self.assertEqual(next(gen), Direction.LEFT)
        self.assertEqual(next(gen), Direction.RIGHT)
        self.assertEqual(next(gen), Direction.LEFT)
        self.assertEqual(next(gen), Direction.RIGHT)
        self.assertEqual(next(gen), Direction.LEFT)

    def test_walk_like_a_ghost(self):
        sol = SolveMap()
        sol.parse_input("./inputs/test_input08.txt")
        self.assertEqual(sol.walk_like_a_ghost(), 6)

    
