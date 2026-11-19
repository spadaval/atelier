import unittest

from slug import slugify


class SlugifyTest(unittest.TestCase):
    def test_lowercases_and_hyphenates(self):
        self.assertEqual(slugify("Hello World"), "hello-world")

    def test_collapses_runs_of_punctuation(self):
        self.assertEqual(slugify("Hello,  World!"), "hello-world")

    def test_trims_hyphens_at_both_ends(self):
        self.assertTrue(slugify("  --Rust & Go--  ").endswith("rust-go"))


if __name__ == "__main__":
    unittest.main()
