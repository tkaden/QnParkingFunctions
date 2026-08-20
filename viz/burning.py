"""Manim scenes for the QnParkingFunctions project.

Render (after `micromamba create -n manim -c conda-forge manim ffmpeg`):

    micromamba run -n manim manim -qm burning.py BurningGame
    micromamba run -n manim manim -qm burning.py SupportClasses

Uses Text (Pango) only — no LaTeX toolchain required.
"""
from manim import (
    Scene, Dot, Line, Text, VGroup, FadeIn, FadeOut, Create, Flash,
    Transform, ORIGIN, UP, DOWN, LEFT, RIGHT, ORANGE, RED, GREEN, BLUE,
    WHITE, GREY, YELLOW, Indicate, Write, LaggedStart,
)

FIRE = ORANGE
COLD = WHITE
ACCENT = BLUE


def square_graph(shift=ORIGIN, scale=1.0):
    """The square Q2: vertices 00 (root, bottom-left), 01 (top-left),
    10 (bottom-right), 11 (top-right)."""
    pos = {
        "00": (LEFT * 1.5 + DOWN * 1.5) * scale + shift,
        "01": (LEFT * 1.5 + UP * 1.5) * scale + shift,
        "10": (RIGHT * 1.5 + DOWN * 1.5) * scale + shift,
        "11": (RIGHT * 1.5 + UP * 1.5) * scale + shift,
    }
    edges = [("00", "01"), ("00", "10"), ("01", "11"), ("10", "11")]
    lines = {e: Line(pos[e[0]], pos[e[1]], color=GREY, stroke_width=3) for e in edges}
    dots = {v: Dot(pos[v], radius=0.14, color=COLD) for v in pos}
    labels = {
        v: Text(v, font_size=26).next_to(
            dots[v],
            (DOWN + LEFT) * 0.5 if v in ("00", "10") else (UP + LEFT) * 0.5,
        )
        for v in pos
    }
    return pos, dots, lines, labels


class BurningGame(Scene):
    """Dhar's burning game on the square, for f = (1,0,0): a parking
    function; then f = (1,1,0): the fire stalls."""

    def burn(self, dot):
        self.play(dot.animate.set_color(FIRE), Flash(dot, color=FIRE), run_time=0.7)

    def setup_graph(self, f_vals, subtitle):
        pos, dots, lines, labels = square_graph(shift=LEFT * 2.5)
        self.add(*lines.values(), *dots.values(), *labels.values())
        res = {}
        for v, val in f_vals.items():
            res[v] = Text(str(val), font_size=30, color=YELLOW).next_to(
                dots[v], RIGHT * 0.6
            )
        title = Text(subtitle, font_size=30).to_edge(UP)
        self.play(Write(title), *[FadeIn(r) for r in res.values()], run_time=1)
        return pos, dots, lines, labels, res, title

    def construct(self):
        rules = VGroup(
            Text("The burning game", font_size=34, color=ACCENT),
            Text("fire starts at the root 00;", font_size=24),
            Text("v ignites when", font_size=24),
            Text("#burning neighbors > f(v)", font_size=24, color=YELLOW),
        ).arrange(DOWN, aligned_edge=LEFT).to_edge(RIGHT).shift(UP * 1.2)
        self.play(FadeIn(rules))

        # ---- round 1: f = (1,0,0) burns completely ----
        pos, dots, lines, labels, res, title = self.setup_graph(
            {"01": 1, "10": 0, "11": 0}, "f = (1, 0, 0)"
        )
        note = Text("root ignites", font_size=24, color=FIRE).to_edge(DOWN)
        self.play(FadeIn(note))
        self.burn(dots["00"])

        for vertex, msg in [
            ("10", "10: one burning neighbor > 0  ✓"),
            ("11", "11: one burning neighbor > 0  ✓"),
            ("01", "01: TWO burning neighbors > 1  ✓"),
        ]:
            new_note = Text(msg, font_size=24, color=FIRE).to_edge(DOWN)
            self.play(Transform(note, new_note), run_time=0.5)
            self.burn(dots[vertex])

        verdict = Text(
            "everything burned — a parking function", font_size=26, color=GREEN
        ).to_edge(DOWN)
        self.play(Transform(note, verdict))
        self.wait(1.2)
        self.play(
            *[FadeOut(m) for m in [*dots.values(), *lines.values(),
                                   *labels.values(), *res.values(), title, note]]
        )

        # ---- round 2: f = (1,1,0) stalls ----
        pos, dots, lines, labels, res, title = self.setup_graph(
            {"01": 1, "10": 1, "11": 0}, "f = (1, 1, 0)"
        )
        note = Text("root ignites", font_size=24, color=FIRE).to_edge(DOWN)
        self.play(FadeIn(note))
        self.burn(dots["00"])

        stall = Text(
            "01 and 10 each need two burning neighbors;", font_size=24, color=RED
        )
        stall2 = Text("11 has none. The fire stalls.", font_size=24, color=RED)
        group = VGroup(stall, stall2).arrange(DOWN).to_edge(DOWN)
        self.play(Transform(note, group))
        self.play(
            Indicate(dots["01"], color=RED),
            Indicate(dots["10"], color=RED),
            Indicate(dots["11"], color=RED),
        )
        verdict = Text("NOT a parking function", font_size=28, color=RED).to_edge(DOWN)
        self.play(Transform(note, verdict))
        self.wait(1.2)

        # ---- the punchline ----
        self.play(*[FadeOut(m) for m in self.mobjects])
        lines_txt = VGroup(
            Text("The square has exactly 4 parking functions:", font_size=30),
            Text("(0,0,0)   (1,0,0)   (0,1,0)   (0,0,1)", font_size=30, color=YELLOW),
            Text("and exactly 4 spanning trees.", font_size=30),
            Text("This is a theorem for every graph:", font_size=30),
            Text("#parking functions = #spanning trees", font_size=34, color=ACCENT),
        ).arrange(DOWN, buff=0.5)
        self.play(LaggedStart(*[FadeIn(t) for t in lines_txt], lag_ratio=0.4))
        self.wait(2)


class SupportClasses(Scene):
    """The square as a prism over one edge: its 4 spanning trees sorted by
    which vertical rungs they use — the support-product theorem in miniature."""

    def prism(self, shift, used_edges, label_text):
        """Draw the square as two levels + rungs; thicken the used edges."""
        a0 = LEFT * 0.8 + DOWN * 0.7 + shift
        b0 = RIGHT * 0.8 + DOWN * 0.7 + shift
        a1 = LEFT * 0.8 + UP * 0.7 + shift
        b1 = RIGHT * 0.8 + UP * 0.7 + shift
        coords = {"a0": a0, "b0": b0, "a1": a1, "b1": b1}
        edges = {
            "h0": ("a0", "b0"), "h1": ("a1", "b1"),
            "ra": ("a0", "a1"), "rb": ("b0", "b1"),
        }
        grp = VGroup()
        for name, (u, v) in edges.items():
            used = name in used_edges
            color = (ACCENT if name in ("ra", "rb") else GREEN) if used else GREY
            grp.add(Line(coords[u], coords[v],
                         stroke_width=6 if used else 2, color=color))
        for c in coords.values():
            grp.add(Dot(c, radius=0.07, color=WHITE))
        grp.add(Text(label_text, font_size=22).next_to(grp, DOWN, buff=0.25))
        return grp

    def construct(self):
        title = Text("4 spanning trees of the square, sorted by rung use",
                     font_size=30).to_edge(UP)
        self.play(Write(title))

        t1 = self.prism(LEFT * 4.6 + UP * 0.6, {"ra", "h0", "h1"}, "rung at a: 1 tree")
        t2 = self.prism(LEFT * 1.4 + UP * 0.6, {"rb", "h0", "h1"}, "rung at b: 1 tree")
        t3 = self.prism(RIGHT * 1.8 + UP * 0.6, {"ra", "rb", "h0"}, "both rungs")
        t4 = self.prism(RIGHT * 4.6 + UP * 0.6, {"ra", "rb", "h1"}, "both rungs: 2 trees")
        self.play(LaggedStart(FadeIn(t1), FadeIn(t2), FadeIn(t3), FadeIn(t4),
                              lag_ratio=0.35))
        self.wait(1)

        law = VGroup(
            Text("support-product theorem:", font_size=28, color=ACCENT),
            Text("#trees using rung set S  =  1/2 · c(base) · 2^|S| · F(S)", font_size=28,
                 color=YELLOW),
            Text("iterate over dimensions  →  Stanley's formula", font_size=26),
        ).arrange(DOWN, buff=0.35).to_edge(DOWN)
        self.play(FadeIn(law))
        self.wait(2)
