"""
Utility functions for image generation in Python.
Replicates functionality from utils.R
"""

import matplotlib.pyplot as plt
import matplotlib
from pathlib import Path

# Color palette adopted for color-blind people
# Based on https://jfly.uni-koeln.de/color/
CBP = {
    'red': '#D55E00',
    'blue': '#56B4E9',
    'green': '#009E73',
    'orange': '#E69F00',
    'navy': '#0072B2',
    'pink': '#CC79A7',
    'yellow': '#F0E442',
    'grey': '#999999'
}


def setup_plot_style(theme='light'):
    """
    Set up matplotlib style to match R's theme_bw() with transparent background.

    Args:
        theme: 'light' or 'dark'
    """
    # Reset to defaults first
    matplotlib.rcParams.update(matplotlib.rcParamsDefault)

    ink_color = 'black' if theme == 'light' else '#CCCAC2'

    # Set all parameters explicitly
    plt.rcParams.update({
        'figure.facecolor': 'none',
        'figure.edgecolor': 'none',
        'axes.facecolor': 'none',
        'axes.edgecolor': ink_color,
        'axes.labelcolor': ink_color,
        'axes.titlecolor': ink_color,
        'axes.grid': True,
        'axes.axisbelow': True,
        'axes.linewidth': 0.8,
        'grid.color': ink_color,
        'grid.alpha': 0.2 if theme == 'light' else 0.3,
        'grid.linewidth': 0.8,
        'savefig.facecolor': 'none',
        'savefig.edgecolor': 'none',
        'savefig.transparent': True,
        'text.color': ink_color,
        'xtick.color': ink_color,
        'xtick.labelcolor': ink_color,
        'ytick.color': ink_color,
        'ytick.labelcolor': ink_color,
        'legend.frameon': True,
        'legend.facecolor': 'white' if theme == 'light' else '#242936',
        'legend.edgecolor': ink_color,
        'legend.framealpha': 0.9,
        'legend.labelcolor': ink_color,
        'font.size': 11,
        'axes.titlesize': 12,
        'axes.labelsize': 11,
        'xtick.labelsize': 10,
        'ytick.labelsize': 10,
        'legend.fontsize': 10,
        'figure.titlesize': 13,
    })


def save_plot(name, plot_func=None, fig=None, multithemed=True, dpi=300, width_px=2400, height_px=1440):
    """
    Save a plot with optional light and dark themes.
    Replicates ggsave_() from utils.R

    Args:
        name: Base name for the file (without extension)
        plot_func: Function that returns a figure (preferred for multithemed)
        fig: matplotlib figure object (if None and plot_func is None, uses current figure)
        multithemed: If True, save both light and dark versions
        dpi: Resolution in dots per inch
        width_px: Width in pixels
        height_px: Height in pixels
    """
    width_inches = width_px / dpi
    height_inches = height_px / dpi

    if multithemed:
        # Save light version
        setup_plot_style('light')
        if plot_func:
            fig = plot_func()
        elif fig is None:
            fig = plt.gcf()
        fig.set_size_inches(width_inches, height_inches)
        filename = f"{name}_light.png"
        fig.savefig(filename, dpi=dpi, bbox_inches='tight', transparent=True)
        print(f"SAVED  : ./{filename}")
        plt.close(fig)

        # Save dark version - recreate the plot with dark theme
        setup_plot_style('dark')
        if plot_func:
            fig = plot_func()
        else:
            # If no plot_func, we need to manually update the existing figure
            fig = plt.gcf()
            dark_color = '#CCCAC2'
            for ax in fig.get_axes():
                ax.set_facecolor('none')
                # Update title
                ax.title.set_color(dark_color)
                # Update axis labels
                ax.xaxis.label.set_color(dark_color)
                ax.yaxis.label.set_color(dark_color)
                # Update tick labels
                for item in ax.get_xticklabels() + ax.get_yticklabels():
                    item.set_color(dark_color)
                # Update spines
                for spine in ax.spines.values():
                    spine.set_edgecolor(dark_color)
                # Update ticks
                ax.tick_params(colors=dark_color)
                # Update grid
                ax.grid(True, color=dark_color, alpha=0.3)
                # Update lines
                for line in ax.get_lines():
                    if line.get_color() == 'black':
                        line.set_color(dark_color)
                # Update legend
                if ax.get_legend():
                    legend = ax.get_legend()
                    legend.get_frame().set_facecolor('#242936')
                    legend.get_frame().set_edgecolor(dark_color)
                    if legend.get_title():
                        legend.get_title().set_color(dark_color)
                    for text in legend.get_texts():
                        text.set_color(dark_color)

        fig.set_size_inches(width_inches, height_inches)
        filename = f"{name}_dark.png"
        fig.savefig(filename, dpi=dpi, bbox_inches='tight', transparent=True)
        print(f"SAVED  : ./{filename}")
        plt.close(fig)
    else:
        setup_plot_style('light')
        if plot_func:
            fig = plot_func()
        elif fig is None:
            fig = plt.gcf()
        fig.set_size_inches(width_inches, height_inches)
        filename = f"{name}.png"
        fig.savefig(filename, dpi=dpi, bbox_inches='tight', transparent=True)
        print(f"SAVED  : ./{filename}")
        plt.close(fig)
