def print_rectangles(x, x_step, max_x, y, y_step, max_y):
    while x <= max_x:
        while y <= max_y:
            print('    {')
            print('      "x": {:.1f},'.format(x))
            print('      "y": {:.1f},'.format(y))
            print('      "w": 16.0,')
            print('      "h": 16.0')
            print('    },')
            y += y_step
        x += x_step

print_rectangles(0.0, 5.0, 320.0, 0.0, 6.0, 6.0)
