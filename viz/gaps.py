"""Gap-filling manim scenes.

    micromamba run -n manim manim -qm gaps.py MaximalStall
    micromamba run -n manim manim -qm gaps.py BoxPartition
    micromamba run -n manim manim -qm gaps.py SpinFlip
"""
from manim import (
    Scene, Dot, Line, Text, VGroup, FadeIn, FadeOut, Transform, Flash,
    RoundedRectangle, Arrow, Cross, ORIGIN, UP, DOWN, LEFT, RIGHT, ORANGE,
    GREEN, BLUE, WHITE, GREY, YELLOW, RED, Write, LaggedStart, Indicate,
)

FIRE, ACCENT, GOOD = ORANGE, BLUE, GREEN


def square(shift=ORIGIN, scale=1.0):
    P = {"00": LEFT * 1.4 * scale + DOWN * 1.2 * scale + shift,
         "01": LEFT * 1.4 * scale + UP * 1.2 * scale + shift,
         "10": RIGHT * 1.4 * scale + DOWN * 1.2 * scale + shift,
         "11": RIGHT * 1.4 * scale + UP * 1.2 * scale + shift}
    E = [("00", "01"), ("00", "10"), ("01", "11"), ("10", "11")]
    lines = VGroup(*[Line(P[u], P[v], color=GREY, stroke_width=3) for u, v in E])
    dots = {v: Dot(P[v], radius=0.13, color=WHITE) for v in P}
    dots["00"].set_color(FIRE)
    labels = VGroup(*[Text(v, font_size=20).next_to(
        dots[v], DOWN * 0.7 if v in ("00", "10") else UP * 0.7) for v in P])
    return P, lines, dots, labels


class MaximalStall(Scene):
    """(1,0,0) burns; raise any entry and the fire stalls: maximality."""

    def show_f(self, P, dots, vals):
        return VGroup(*[Text(str(vals[v]), font_size=30, color=YELLOW)
                        .next_to(dots[v], RIGHT * 0.6)
                        for v in ("01", "10", "11")])

    def reset(self, dots):
        for v in ("01", "10", "11"):
            dots[v].set_color(WHITE)

    def construct(self):
        title = Text("maximal parking functions", font_size=32,
                     color=ACCENT).to_edge(UP)
        self.play(Write(title))
        P, lines, dots, labels = square(shift=LEFT * 2.8)
        self.play(FadeIn(lines), FadeIn(VGroup(*dots.values())), FadeIn(labels))

        cap = Text("f = (1,0,0): everything burns ✓", font_size=26,
                   color=GOOD).to_edge(DOWN)
        f = self.show_f(P, dots, {"01": 1, "10": 0, "11": 0})
        self.play(FadeIn(f), FadeIn(cap))
        for v in ("10", "11", "01"):
            self.play(dots[v].animate.set_color(FIRE), Flash(dots[v], color=FIRE),
                      run_time=0.5)
        self.wait(0.6)

        trials = [
            ({"01": 2, "10": 0, "11": 0}, ["10", "11"], "01",
             "raise the 1 to 2:  01 now needs three burning neighbors — stall ✗"),
            ({"01": 1, "10": 1, "11": 0}, [], "10",
             "raise a 0 to 1:  nothing after the root can ignite — stall ✗"),
            ({"01": 1, "10": 0, "11": 1}, ["10"], "11",
             "raise the other 0:  the fire dies at 11 — stall ✗"),
        ]
        panel = VGroup()
        for vals, burns, stall, msg in trials:
            self.reset(dots)
            f2 = self.show_f(P, dots, vals)
            cap2 = Text(msg, font_size=24, color=RED).to_edge(DOWN)
            self.play(Transform(f, f2), Transform(cap, cap2), run_time=0.7)
            for v in burns:
                self.play(dots[v].animate.set_color(FIRE), run_time=0.35)
            x = Cross(dots[stall], stroke_color=RED, stroke_width=5,
                      scale_factor=1.6)
            self.play(FadeIn(x), run_time=0.4)
            self.wait(0.6)
            self.play(FadeOut(x), run_time=0.2)

        self.reset(dots)
        finale = VGroup(
            Text("(1,0,0) cannot be raised anywhere: it is MAXIMAL.",
                 font_size=26),
            Text("the square has three:  (1,0,0)  (0,1,0)  (0,0,1)",
                 font_size=26, color=YELLOW),
            Text("each sums to 1 = edges − vertices + 1 — never an accident",
                 font_size=26, color=ACCENT),
        ).arrange(DOWN, buff=0.3).to_edge(DOWN).shift(UP * 0.1)
        self.play(Transform(cap, finale))
        self.wait(2.2)


class BoxPartition(Scene):
    """Canonical burn orders tile the parking functions into boxes."""

    def burn_seq(self, dots, order, note_holder, msgs):
        anims = []
        for i, v in enumerate(order):
            self.play(dots[v].animate.set_color(FIRE), run_time=0.4)

    def construct(self):
        title = Text("the box partition", font_size=32, color=ACCENT).to_edge(UP)
        self.play(Write(title))
        P, lines, dots, labels = square(shift=LEFT * 3.4, scale=0.9)
        idx = Text("labels: 00→0  01→1  10→2  11→3", font_size=20,
                   color=GREY).next_to(title, DOWN)
        self.play(FadeIn(lines), FadeIn(VGroup(*dots.values())), FadeIn(labels),
                  FadeIn(idx))

        rule = Text("rule: always burn the smallest burnable label",
                    font_size=24, color=YELLOW).to_edge(DOWN)
        self.play(FadeIn(rule))

        # boxes on the right
        bx = RoundedRectangle(corner_radius=0.15, width=3.1, height=1.7,
                              color=ACCENT).shift(RIGHT * 3.6 + UP * 1.3)
        bx_lab = Text("order 0,1,2,3", font_size=18, color=GREY).next_to(bx, UP)
        b2 = RoundedRectangle(corner_radius=0.15, width=3.1, height=0.85,
                              color=ACCENT).shift(RIGHT * 3.6 + DOWN * 0.55)
        b2_lab = Text("order 0,2,3,1", font_size=18, color=GREY).next_to(
            b2, RIGHT).rotate(0)
        b3 = RoundedRectangle(corner_radius=0.15, width=3.1, height=0.85,
                              color=ACCENT).shift(RIGHT * 3.6 + DOWN * 1.85)
        b3_lab = Text("order 0,1,3,2", font_size=18, color=GREY).next_to(
            b3, RIGHT)

        def run(vals, order, entry_text, target, note):
            f = VGroup(*[Text(str(vals[v]), font_size=26, color=YELLOW)
                         .next_to(dots[v], RIGHT * 0.55)
                         for v in ("01", "10", "11")])
            cap = Text(note, font_size=22).to_edge(DOWN)
            self.play(FadeIn(f), Transform(rule, cap), run_time=0.6)
            for v in ("01", "10", "11"):
                dots[v].set_color(WHITE)
            for v in order:
                self.play(dots[v].animate.set_color(FIRE), run_time=0.3)
            tag = Text(entry_text, font_size=22, color=YELLOW).move_to(target)
            self.play(f.animate.become(tag), run_time=0.7)
            return f

        self.play(FadeIn(bx), FadeIn(bx_lab))
        f1 = run({"01": 0, "10": 0, "11": 0}, ["01", "10", "11"],
                 "(0,0,0)", bx.get_center() + DOWN * 0.4,
                 "(0,0,0): burns 0,1,2,3 — smallest first")
        f2 = run({"01": 0, "10": 0, "11": 1}, ["01", "10", "11"],
                 "(0,0,1) ★", bx.get_center() + UP * 0.4,
                 "(0,0,1): the SAME order 0,1,2,3 — same box!")
        self.play(FadeIn(b2), FadeIn(b2_lab))
        f3 = run({"01": 1, "10": 0, "11": 0}, ["10", "11", "01"],
                 "(1,0,0) ★", b2.get_center(),
                 "(1,0,0): vertex 1 resists — the fire skips to 2. New box.")
        self.play(FadeIn(b3), FadeIn(b3_lab))
        f4 = run({"01": 0, "10": 1, "11": 0}, ["01", "11", "10"],
                 "(0,1,0) ★", b3.get_center(),
                 "(0,1,0): the mirror skip — a third box.")

        finale = VGroup(
            Text("the boxes TILE all parking functions; the tops ★ are", font_size=24),
            Text("exactly the maximal ones. Box sizes sum to #trees: 2+1+1 = 4.",
                 font_size=24, color=YELLOW),
        ).arrange(DOWN, buff=0.15).to_edge(DOWN)
        self.play(Transform(rule, finale))
        self.wait(2.5)


class SpinFlip(Scene):
    """The atomic move: a pendant fiber's attaching edge changes level;
    exactly one spin flips; everything reverses."""

    def construct(self):
        title = Text("the spin flip (the atomic move)", font_size=32,
                     color=ACCENT).to_edge(UP)
        self.play(Write(title))

        # prism over the edge {a, b}: a0,b0 bottom; a1,b1 top
        A0, B0 = LEFT * 2.2 + DOWN * 1.3, RIGHT * 2.2 + DOWN * 1.3
        A1, B1 = LEFT * 2.2 + UP * 1.3, RIGHT * 2.2 + UP * 1.3
        lv0 = Text("level 0", font_size=18, color=GREY).next_to(A0, LEFT, buff=0.7)
        lv1 = Text("level 1", font_size=18, color=GREY).next_to(A1, LEFT, buff=0.7)
        ra = Line(A0, A1, color=ACCENT, stroke_width=6)
        rb = Line(B0, B1, color=ACCENT, stroke_width=6)
        h1 = Line(A1, B1, color=GOOD, stroke_width=6)
        h0_ghost = Line(A0, B0, color=GREY, stroke_width=1.5, stroke_opacity=0.35)
        dots = VGroup(Dot(A0, radius=0.09), Dot(A1, radius=0.09),
                      Dot(B1, radius=0.09), Dot(B0, radius=0.15, color=FIRE))
        names = VGroup(Text("a0", font_size=18).next_to(A0, DOWN),
                       Text("a1", font_size=18).next_to(A1, UP),
                       Text("b1", font_size=18).next_to(B1, UP),
                       Text("root", font_size=18, color=FIRE).next_to(B0, DOWN))
        self.play(FadeIn(VGroup(h0_ghost, ra, rb, h1, dots, names, lv0, lv1)))

        sa = Text("σ(a) = 1", font_size=24, color=RED).next_to(ra, LEFT)
        sb = Text("σ(b) = 0", font_size=24, color=GOOD).next_to(rb, RIGHT)
        cap = VGroup(
            Text("rung a's closer-to-root end is a1, on level 1: spin 1.", font_size=24),
            Text("fiber a hangs by ONE edge — a pendant. Watch:", font_size=24),
        ).arrange(DOWN, buff=0.12).to_edge(DOWN)
        self.play(FadeIn(sa), FadeIn(sb), FadeIn(cap))
        self.wait(1.2)

        # the flip: h1 slides to h0
        h_new = Line(A0, B0, color=GOOD, stroke_width=6)
        h1_ghost = Line(A1, B1, color=GREY, stroke_width=1.5, stroke_opacity=0.35)
        sa2 = Text("σ(a) = 0", font_size=24, color=GOOD).next_to(ra, LEFT)
        cap2 = VGroup(
            Text("the attaching edge moves to the other level — nothing else", font_size=24),
            Text("changes, and rung a's spin flips. Perfectly reversible.", font_size=24),
        ).arrange(DOWN, buff=0.12).to_edge(DOWN)
        self.play(Transform(h1, h_new), Transform(h0_ghost, h1_ghost),
                  Transform(sa, sa2), Transform(cap, cap2), run_time=1.4)
        self.wait(1.4)

        cap3 = VGroup(
            Text("rungs that are NOT pendant reduce to this case by a", font_size=24),
            Text("shrink-and-restore recursion (Bernardi 2012) — so every", font_size=24),
            Text("switch can be set, one at a time, without touching the rest.", font_size=24),
        ).arrange(DOWN, buff=0.12).to_edge(DOWN)
        self.play(Transform(cap, cap3))
        self.wait(2.2)
