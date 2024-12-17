def print_rectangles(x, x_step, max_x, y, y_step, max_y):
    while x <= max_x:
        y = 0.0
        while y <= max_y:
            print('    {')
            print('      "x": {:.1f},'.format(x))
            print('      "y": {:.1f},'.format(y))
            print('      "w": 1560.0,')
            print('      "h": 1560.0')
            print('    },')
            y += y_step
        x += x_step

print_rectangles(0.0, 1560.0, 4680.0, 0.0, 1560.0, 0.0)
