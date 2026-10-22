#!/usr/bin/env python3
"""Convert Solar Icons SVGs into mirui `path!` literals.

Solar ships its icons as 24x24 SVGs with a single `fill-rule="evenodd"` path.
mirui's `path!` macro understands M/L/Q/C/Z on a 24x24 grid, so this script
parses the SVG path data, makes it absolute, turns arcs into cubic segments and
prints a Rust constant per icon.

Usage:  python3 tools/solar_to_mirui.py <icons-dir> <name> [<name> ...]
"""

import math
import re
import sys
import xml.etree.ElementTree as ET

NUMBER = re.compile(r"[-+]?(?:\d*\.\d+|\d+\.?)(?:[eE][-+]?\d+)?")
COMMAND = re.compile(r"([MmLlHhVvCcSsQqTtAaZz])")


def tokenize(data):
    """Yield (command, [numbers]) pairs."""
    parts = []
    pos = 0
    for match in COMMAND.finditer(data):
        if match.start() > pos:
            parts.append((None, NUMBER.findall(data[pos:match.start()])))
        parts.append((match.group(1), []))
        pos = match.end()
    if pos < len(data):
        parts.append((None, NUMBER.findall(data[pos:])))

    out = []
    command = None
    for cmd, numbers in parts:
        if cmd is not None:
            command = cmd
            out.append((command, []))
        else:
            for i in range(0, len(numbers), 1):
                out[-1][1].append(float(numbers[i]))
    return out


def arc_to_cubics(x0, y0, rx, ry, phi, large, sweep, x, y):
    """Endpoint arc to centre parameterisation, then to cubic segments."""
    if rx == 0 or ry == 0 or (x0 == x and y0 == y):
        return [("L", [x, y])]
    rx, ry = abs(rx), abs(ry)
    phi = math.radians(phi)
    cos_phi, sin_phi = math.cos(phi), math.sin(phi)
    dx2, dy2 = (x0 - x) / 2.0, (y0 - y) / 2.0
    x1 = cos_phi * dx2 + sin_phi * dy2
    y1 = -sin_phi * dx2 + cos_phi * dy2
    lam = (x1 * x1) / (rx * rx) + (y1 * y1) / (ry * ry)
    if lam > 1:
        scale = math.sqrt(lam)
        rx, ry = rx * scale, ry * scale
    num = rx * rx * ry * ry - rx * rx * y1 * y1 - ry * ry * x1 * x1
    den = rx * rx * y1 * y1 + ry * ry * x1 * x1
    factor = math.sqrt(max(0.0, num / den)) if den else 0.0
    if large == sweep:
        factor = -factor
    cx1 = factor * rx * y1 / ry
    cy1 = -factor * ry * x1 / rx
    cx = cos_phi * cx1 - sin_phi * cy1 + (x0 + x) / 2.0
    cy = sin_phi * cx1 + cos_phi * cy1 + (y0 + y) / 2.0

    def angle(ux, uy, vx, vy):
        dot = ux * vx + uy * vy
        length = math.hypot(ux, uy) * math.hypot(vx, vy)
        if length == 0:
            return 0.0
        value = max(-1.0, min(1.0, dot / length))
        result = math.acos(value)
        if ux * vy - uy * vx < 0:
            result = -result
        return result

    theta1 = angle(1, 0, (x1 - cx1) / rx, (y1 - cy1) / ry)
    dtheta = angle((x1 - cx1) / rx, (y1 - cy1) / ry, (-x1 - cx1) / rx, (-y1 - cy1) / ry)
    if not sweep and dtheta > 0:
        dtheta -= 2 * math.pi
    elif sweep and dtheta < 0:
        dtheta += 2 * math.pi

    segments = max(1, int(math.ceil(abs(dtheta) / (math.pi / 2))))
    delta = dtheta / segments
    t = 4.0 / 3.0 * math.tan(delta / 4.0)
    result = []
    theta = theta1
    px, py = x0, y0
    for _ in range(segments):
        theta2 = theta + delta
        cos1, sin1 = math.cos(theta), math.sin(theta)
        cos2, sin2 = math.cos(theta2), math.sin(theta2)
        ex = cx + rx * cos_phi * cos2 - ry * sin_phi * sin2
        ey = cy + rx * sin_phi * cos2 + ry * cos_phi * sin2
        dx1 = -rx * cos_phi * sin1 - ry * sin_phi * cos1
        dy1 = -rx * sin_phi * sin1 + ry * cos_phi * cos1
        dx2 = -rx * cos_phi * sin2 - ry * sin_phi * cos2
        dy2 = -rx * sin_phi * sin2 + ry * cos_phi * cos2
        c1x = px + t * dx1
        c1y = py + t * dy1
        c2x = ex - t * dx2
        c2y = ey - t * dy2
        result.append(("C", [c1x, c1y, c2x, c2y, ex, ey]))
        px, py = ex, ey
        theta = theta2
    return result


def to_absolute(steps):
    """Normalise a path into absolute M/L/C/Q/Z steps."""
    out = []
    x = y = 0.0
    start_x = start_y = 0.0
    last_c = None
    last_q = None
    for command, args in steps:
        upper = command.upper()
        relative = command.islower()
        if upper == "M":
            for index in range(0, len(args), 2):
                nx, ny = args[index], args[index + 1]
                if relative:
                    nx, ny = x + nx, y + ny
                if index == 0:
                    out.append(("M", [nx, ny]))
                    x, y = nx, ny
                    start_x, start_y = nx, ny
                else:
                    out.append(("L", [nx, ny]))
                    x, y = nx, ny
            last_c = last_q = None
        elif upper == "L":
            for index in range(0, len(args), 2):
                nx, ny = args[index], args[index + 1]
                if relative:
                    nx, ny = x + nx, y + ny
                out.append(("L", [nx, ny]))
                x, y = nx, ny
            last_c = last_q = None
        elif upper == "H":
            for value in args:
                nx = x + value if relative else value
                out.append(("L", [nx, y]))
                x = nx
            last_c = last_q = None
        elif upper == "V":
            for value in args:
                ny = y + value if relative else value
                out.append(("L", [x, ny]))
                y = ny
            last_c = last_q = None
        elif upper == "C":
            for index in range(0, len(args), 6):
                chunk = args[index:index + 6]
                if relative:
                    chunk = [chunk[0] + x, chunk[1] + y, chunk[2] + x, chunk[3] + y, chunk[4] + x, chunk[5] + y]
                out.append(("C", chunk))
                x, y = chunk[4], chunk[5]
                last_c = (chunk[0], chunk[1])
            last_q = None
        elif upper == "S":
            for index in range(0, len(args), 4):
                chunk = args[index:index + 4]
                if relative:
                    chunk = [chunk[0] + x, chunk[1] + y, chunk[2] + x, chunk[3] + y]
                if last_c is None:
                    c1 = (x, y)
                else:
                    c1 = (2 * x - last_c[0], 2 * y - last_c[1])
                out.append(("C", [c1[0], c1[1], chunk[0], chunk[1], chunk[2], chunk[3]]))
                x, y = chunk[2], chunk[3]
                last_c = (chunk[0], chunk[1])
            last_q = None
        elif upper == "Q":
            for index in range(0, len(args), 4):
                chunk = args[index:index + 4]
                if relative:
                    chunk = [chunk[0] + x, chunk[1] + y, chunk[2] + x, chunk[3] + y]
                out.append(("Q", chunk))
                x, y = chunk[2], chunk[3]
                last_q = (chunk[0], chunk[1])
            last_c = None
        elif upper == "T":
            for index in range(0, len(args), 2):
                nx, ny = args[index], args[index + 1]
                if relative:
                    nx, ny = x + nx, y + ny
                if last_q is None:
                    q = (x, y)
                else:
                    q = (2 * x - last_q[0], 2 * y - last_q[1])
                out.append(("Q", [q[0], q[1], nx, ny]))
                x, y = nx, ny
                last_q = q
            last_c = None
        elif upper == "A":
            for index in range(0, len(args), 7):
                chunk = args[index:index + 7]
                if relative:
                    chunk[5] = x + chunk[5]
                    chunk[6] = y + chunk[6]
                for step in arc_to_cubics(x, y, *chunk):
                    out.append(step)
                x, y = chunk[5], chunk[6]
            last_c = last_q = None
        elif upper == "Z":
            out.append(("Z", []))
            x, y = start_x, start_y
            last_c = last_q = None
    return out


def format_number(value):
    rounded = round(value, 2)
    if abs(rounded - round(rounded)) < 0.005:
        return str(int(round(rounded)))
    return f"{rounded:g}"


def to_mirui(steps):
    parts = []
    for command, args in steps:
        if command == "Z":
            parts.append("Z")
        else:
            numbers = " ".join(format_number(value) for value in args)
            parts.append(f"{command} {numbers}")
    return " ".join(parts)


def main():
    directory = sys.argv[1]
    names = sys.argv[2:]
    for name in names:
        path = f"{directory}/{name}.svg"
        tree = ET.parse(path)
        root = tree.getroot()
        # An icon can be several <path> elements. mirui has a single path with
        # one fill rule, so the pieces are concatenated; the even-odd rule then
        # behaves like the per-path rule for the shapes that do not overlap.
        data = None
        for node in root.iter():
            if node.tag.endswith("path") and node.get("d"):
                piece = node.get("d")
                data = piece if data is None else data + " " + piece
        if data is None:
            print(f"// {name}: no path")
            continue
        steps = to_absolute(tokenize(data))
        print(f"// {name}")
        print(f"pub static {name.upper().replace('-', '_')}: Path = path!({to_mirui(steps)});")
        print()


if __name__ == "__main__":
    main()
