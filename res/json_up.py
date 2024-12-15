def print_rectangles(x_step, max_x, y):
    x = 0.0
    while x <= max_x:
        print('    {')
        print('      "x": {:.1f},'.format(x))
        print('      "y": {:.1f},'.format(y))
        print('      "w": 16.0,')
        print('      "h": 16.0')
        print('    },')
        x += x_step

print_rectangles(16.0, 320.0, 48.0)
