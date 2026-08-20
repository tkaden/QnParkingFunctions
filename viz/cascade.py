"""Manim scenes for the bijective proof.

    micromamba run -n manim manim -qm cascade.py StarCollapse
    micromamba run -n manim manim -qm cascade.py PruferCascade

The PruferCascade instance is real: computed by the verified bijection
(qpark/scripts/bijection.py). Tree parents [1,3,3,7,5,7,7,-], root 111,
code (1,1,3,1,3,3,5) in the box [2][2][4][2][4][4][6].
"""
from manim import (
    Scene, Dot, Line, Text, VGroup, FadeIn, FadeOut, Create, Transform,
    ReplacementTransform, RoundedRectangle, Arrow, ORIGIN, UP, DOWN, LEFT,
    RIGHT, ORANGE, GREEN, BLUE, WHITE, GREY, YELLOW, PURPLE, TEAL, RED,
    Write, LaggedStart, Indicate, config,
)

ACCENT, GOOD, FIRE = BLUE, GREEN, ORANGE

# ---------- the real instance ----------
Q3_PAR = [1, 3, 3, 7, 5, 7, 7, -1]
Q3_ROOT = 7
CODE = [1, 1, 3, 1, 3, 3, 5]
DOMS = [2, 2, 4, 2, 4, 4, 6]
A2_PAR, A2_ROOT = [1, 3, 3, -1], 3            # rooted tree of the square
B3_PAR, B3_ROOT, B3_COL = [1, 3, 3, -1], 3, 1  # 2-colored forest of the square
A1_PAR, A1_ROOT = [1, -1], 1                   # rooted tree of the edge
B2_PAR, B2_ROOT, B2_COL = [1, -1], 1, 1
B1_COL = CODE[0]

CHIP_COLORS = {2: TEAL, 4: BLUE, 6: PURPLE}


def cube_positions(scale=1.0, shift=ORIGIN):
    """Vertices of Q3 drawn as outer/inner square; bit2 = inner/outer."""
    pos = {}
    for v in range(8):
        x = (1.5 if v & 1 else -1.5) * scale
        y = (1.2 if v & 2 else -1.2) * scale
        if v & 4:
            x, y = x * 0.5 + 0.55 * scale, y * 0.5 + 0.45 * scale
        pos[v] = [x + shift[0], y + shift[1], 0]
    return pos


def square_positions(scale=1.0, shift=ORIGIN):
    pos = {}
    for v in range(4):
        x = (1.0 if v & 1 else -1.0) * scale
        y = (0.9 if v & 2 else -0.9) * scale
        pos[v] = [x + shift[0], y + shift[1], 0]
    return pos


def graph_group(n, edges, tree_edges, pos, root=None, root_color=FIRE,
                labels=None, rung_bit=None):
    g = VGroup()
    for (u, v) in edges:
        used = (u, v) in tree_edges or (v, u) in tree_edges
        is_rung = rung_bit is not None and (u ^ v) == rung_bit
        color = (ACCENT if is_rung else GOOD) if used else GREY
        g.add(Line(pos[u], pos[v], color=color,
                   stroke_width=6 if used else 1.5,
                   stroke_opacity=1 if used else 0.35))
    for v in range(n):
        if root is not None and v == root:
            g.add(Dot(pos[v], radius=0.14, color=root_color))
        else:
            g.add(Dot(pos[v], radius=0.07, color=WHITE))
    if labels:
        for v, name in labels.items():
            g.add(Text(name, font_size=18).move_to(
                [pos[v][0], pos[v][1] + 0.32, 0]))
    return g


def chip(value, dom, height=0.55):
    box = RoundedRectangle(corner_radius=0.1, width=0.62, height=height,
                           color=CHIP_COLORS[dom], fill_opacity=0.25)
    t = Text(str(value), font_size=26).move_to(box)
    d = Text(f"/{dom}", font_size=14, color=GREY).next_to(box, DOWN, buff=0.08)
    return VGroup(box, t, d)


class StarCollapse(Scene):
    """The all-spins-0 theorem: bipartite component tree collapses to a star."""

    def construct(self):
        title = Text("the all-spins-0 theorem", font_size=32, color=ACCENT).to_edge(UP)
        self.play(Write(title))
        lv0 = Line(LEFT * 6 + DOWN * 1.8, RIGHT * 6 + DOWN * 1.8, color=GREY,
                   stroke_opacity=0.4)
        lv1 = Line(LEFT * 6 + UP * 1.8, RIGHT * 6 + UP * 1.8, color=GREY,
                   stroke_opacity=0.4)
        l0 = Text("level 0", font_size=20, color=GREY).next_to(lv0, DOWN)
        l1 = Text("level 1", font_size=20, color=GREY).next_to(lv1, UP)
        self.play(FadeIn(lv0), FadeIn(lv1), FadeIn(l0), FadeIn(l1))

        # a component tree: blobs alternating levels, edges = rungs
        P = {0: DOWN * 1.8 + LEFT * 3, 1: UP * 1.8 + LEFT * 1.4,
             2: DOWN * 1.8 + RIGHT * 0.4, 3: UP * 1.8 + RIGHT * 2.2,
             4: DOWN * 1.8 + RIGHT * 3.8}
        blobs = VGroup(*[
            RoundedRectangle(corner_radius=0.25, width=1.1, height=0.7,
                             color=(ACCENT if k == 0 else WHITE)).move_to(P[k])
            for k in range(5)])
        root_tag = Text("root", font_size=20, color=FIRE).next_to(blobs[0], DOWN)
        bedges = [(0, 1), (1, 2), (2, 3), (3, 4)]
        arrows = VGroup(*[
            Arrow(P[b], P[a], buff=0.5, color=YELLOW, stroke_width=4)
            for a, b in bedges])
        cap = Text("rungs point toward the root: spins say which level the",
                   font_size=24)
        cap2 = Text("closer end is on — set every spin to 0 …", font_size=24)
        caps = VGroup(cap, cap2).arrange(DOWN, buff=0.1).to_edge(DOWN)
        self.play(FadeIn(blobs), FadeIn(root_tag), LaggedStart(
            *[Create(a) for a in arrows], lag_ratio=0.3), FadeIn(caps))
        self.wait(1.2)

        # collapse: one level-0 blob, three level-1 leaves
        Pc = {0: DOWN * 1.5, 1: UP * 1.8 + LEFT * 2.4, 2: UP * 1.8,
              3: UP * 1.8 + RIGHT * 2.4}
        big = RoundedRectangle(corner_radius=0.25, width=3.4, height=0.9,
                               color=ACCENT, fill_opacity=0.2).move_to(Pc[0])
        leaves = VGroup(*[
            RoundedRectangle(corner_radius=0.25, width=1.0, height=0.65,
                             color=GOOD).move_to(Pc[k]) for k in (1, 2, 3)])
        new_arrows = VGroup(*[
            Arrow(Pc[k], Pc[0] + UP * 0.4, buff=0.35, color=YELLOW,
                  stroke_width=4) for k in (1, 2, 3)])
        newcap = VGroup(
            Text("… and the picture is forced: one spanning tree flat on", font_size=24),
            Text("level 0, forest pieces hanging by single rungs. Frozen.", font_size=24),
        ).arrange(DOWN, buff=0.1).to_edge(DOWN)
        tlab = Text("tree T", font_size=22).move_to(big)
        flab = Text("forest F", font_size=20, color=GOOD).next_to(leaves[2], RIGHT)
        self.play(ReplacementTransform(blobs, VGroup(big, leaves)),
                  ReplacementTransform(arrows, new_arrows),
                  Transform(caps, newcap), FadeIn(tlab), FadeIn(flab),
                  root_tag.animate.next_to(big, DOWN))
        self.wait(2)


class PruferCascade(Scene):
    """One real rooted spanning tree of the cube, cascading to its code."""

    def rack(self):
        chips = VGroup(*[chip(CODE[i], DOMS[i]) for i in range(7)])
        chips.arrange(RIGHT, buff=0.25).to_edge(DOWN).shift(UP * 0.15)
        return chips

    def construct(self):
        title = Text("a Prüfer code for the cube", font_size=32,
                     color=ACCENT).to_edge(UP)
        self.play(Write(title))

        # rack silhouette
        rack = self.rack()
        silhouettes = VGroup(*[
            RoundedRectangle(corner_radius=0.1, width=0.62, height=0.55,
                             color=GREY, stroke_opacity=0.5).move_to(rack[i][0])
            for i in range(7)])
        self.play(FadeIn(silhouettes))

        # the cube tree
        cpos = cube_positions(scale=1.35, shift=UP * 0.7 + LEFT * 3.2)
        cedges = [(v, v ^ (1 << b)) for v in range(8) for b in range(3)
                  if v < (v ^ (1 << b))]
        ctree = [(v, Q3_PAR[v]) for v in range(8) if Q3_PAR[v] >= 0]
        cube = graph_group(8, cedges, ctree, cpos, root=Q3_ROOT, rung_bit=4,
                           labels={7: "root 111"})
        cap = Text("one of the cube's 3 072 rooted spanning trees",
                   font_size=24).next_to(silhouettes, UP, buff=0.5)
        self.play(FadeIn(cube), FadeIn(cap))
        self.wait(1.2)

        # step 1: cube -> square tree + colored forest
        spos_a = square_positions(scale=1.0, shift=UP * 1.1 + LEFT * 4.6)
        spos_b = square_positions(scale=1.0, shift=UP * 1.1 + RIGHT * 0.2)
        sedges = [(v, v ^ (1 << b)) for v in range(4) for b in range(2)
                  if v < (v ^ (1 << b))]
        a2 = graph_group(4, sedges, [(v, A2_PAR[v]) for v in range(4)
                                     if A2_PAR[v] >= 0], spos_a, root=A2_ROOT)
        b3 = graph_group(4, sedges, [(v, B3_PAR[v]) for v in range(4)
                                     if B3_PAR[v] >= 0], spos_b, root=B3_ROOT,
                         root_color=TEAL)
        albl = Text("smaller rooted tree", font_size=20).next_to(a2, DOWN)
        blbl = Text("colored forest (spins!)", font_size=20,
                    color=TEAL).next_to(b3, DOWN)
        newcap = Text("freeze the spins  →  a square tree  +  a colored forest",
                      font_size=24).next_to(silhouettes, UP, buff=0.5)
        self.play(ReplacementTransform(cube, VGroup(a2, b3)),
                  Transform(cap, newcap), FadeIn(albl), FadeIn(blbl))
        self.wait(1.2)

        # forest branch sublimates into chips 4..7
        ch = VGroup(*[chip(CODE[i], DOMS[i]) for i in range(3, 7)])
        for i, c in enumerate(ch):
            c.move_to(silhouettes[3 + i][0])
        subcap = Text("the same machine eats the forest → four dial values",
                      font_size=24).next_to(silhouettes, UP, buff=0.5)
        self.play(FadeOut(b3, shift=DOWN * 0.4), FadeOut(blbl),
                  LaggedStart(*[FadeIn(c, shift=DOWN * 0.5) for c in ch],
                              lag_ratio=0.15),
                  Transform(cap, subcap))
        self.wait(1.2)

        # step 2: square tree -> edge tree + colored forest
        epos_a = {0: UP * 1.1 + LEFT * 5.4, 1: UP * 1.1 + LEFT * 3.9}
        epos_b = {0: UP * 1.1 + LEFT * 1.4, 1: UP * 1.1 + RIGHT * 0.1}
        a1 = graph_group(2, [(0, 1)], [(0, 1)], epos_a, root=A1_ROOT)
        b2 = graph_group(2, [(0, 1)], [(0, 1)], epos_b, root=B2_ROOT,
                         root_color=TEAL)
        self.play(ReplacementTransform(VGroup(a2, albl), VGroup(a1, b2)),
                  Transform(cap, Text("again: edge tree + colored forest",
                                      font_size=24).next_to(silhouettes, UP,
                                                            buff=0.5)))
        ch2 = VGroup(*[chip(CODE[i], DOMS[i]) for i in range(1, 3)])
        for i, c in enumerate(ch2):
            c.move_to(silhouettes[1 + i][0])
        self.play(FadeOut(b2, shift=DOWN * 0.4),
                  LaggedStart(*[FadeIn(c, shift=DOWN * 0.5) for c in ch2],
                              lag_ratio=0.15))
        self.wait(0.8)

        # step 3: edge -> dot + final chip
        ch1 = chip(CODE[0], DOMS[0]).move_to(silhouettes[0][0])
        self.play(FadeOut(a1, shift=DOWN * 0.4), FadeIn(ch1, shift=DOWN * 0.5),
                  Transform(cap, Text("…down to a single dot and the last dial",
                                      font_size=24).next_to(silhouettes, UP,
                                                            buff=0.5)))
        self.wait(0.8)

        final = VGroup(
            Text("code (1,1,3,1,3,3,5) — and every step reverses.", font_size=26),
            Text("2·2·4·2·4·4·6 = 3 072 codes = 3 072 rooted trees", font_size=26,
                 color=YELLOW),
            Text("∏(2k)^C(n,k): Stanley's formula, proved by matching up.",
                 font_size=26, color=ACCENT),
        ).arrange(DOWN, buff=0.3).move_to(UP * 1.2)
        self.play(Transform(cap, final))
        self.wait(2.5)
