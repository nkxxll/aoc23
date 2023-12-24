def main():
    with open("inputs/test_input11.txt") as f:
        lines = f.readlines()
        if lines[-1] == "":
            lines = lines[:-1]
    newlines = []
    for line in lines:
        if all([c in "." for c in line.strip()]):
            newlines.append(line)
        newlines.append(line)

    # TODO: expand the x axis as well

    points = {}
    counter = 0
    for y, line in enumerate(newlines):
        for x, c in enumerate(line):
            if c == "#":
                points[counter] = (x, y)
                counter += 1
    print(points)
            

if __name__ == "__main__":
    main()
