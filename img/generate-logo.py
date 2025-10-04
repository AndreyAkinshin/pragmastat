#!/usr/bin/env python3
"""
Generate the Pragmastat logo.
Python implementation of generate-logo.R
"""

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.patches as patches
from scipy import stats
from utils import CBP


def figure_logo():
    """
    Generate the Pragmastat logo.
    Creates a blue circle with a complex statistical curve overlay.
    """
    # Parameters matching R code
    l, r = 0, 18
    w = r - l
    x = np.linspace(l, r, 10000)

    # Complex curve from multiple normal distributions and sine waves
    y = (1 * stats.norm.pdf(x, loc=3, scale=1) +
         2 * stats.norm.pdf(x, loc=8, scale=1) +
         2 * stats.norm.pdf(x, loc=5, scale=5) +
         0.02 * np.sin(x * 2) +
         0.015 * np.sin(x * 5))

    # Normalize coordinates to [0, 1] range with padding
    x = 0.05 + x / r * 0.9
    y = 0.15 + y / np.max(y) * 0.8

    # Create figure with transparent background
    fig = plt.figure(figsize=(8, 8))
    fig.patch.set_alpha(0.0)

    ax = fig.add_subplot(111)
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    ax.set_aspect('equal')
    ax.axis('off')

    # Draw blue circle background
    c = 0.5
    radius = 0.5
    circle = patches.Circle((c, c), radius, facecolor=CBP['blue'],
                           edgecolor='none', zorder=1)
    ax.add_patch(circle)

    # Filter points inside the circle (with small margin)
    distances = np.sqrt((x - c)**2 + (y - c)**2)
    inside_circle = distances <= (radius - 0.02)

    # Draw the curve inside the circle
    x_inside = x[inside_circle]
    y_inside = y[inside_circle]

    # Draw with thick black line
    ax.plot(x_inside, y_inside, color='black', linewidth=50,
           solid_capstyle='round', solid_joinstyle='round', zorder=2)

    return fig


if __name__ == '__main__':
    fig = figure_logo()

    # Save as PNG with transparent background at 800x800
    fig.savefig('logo.png', dpi=100, bbox_inches='tight',
               pad_inches=0, transparent=True, facecolor='none')

    print("SAVED  : ./logo.png")

    plt.close(fig)
