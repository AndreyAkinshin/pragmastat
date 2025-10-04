#!/usr/bin/env python3
"""
Generate statistical charts from JSON data.
Python implementation of generate-images.R
"""

import json
import numpy as np
import matplotlib.pyplot as plt
from pathlib import Path
import glob
from scipy import stats
from utils import CBP, setup_plot_style, save_plot


def load_json(filepath):
    """Load JSON data from file."""
    with open(filepath, 'r') as f:
        return json.load(f)


def generate_avg_drift():
    """Generate average drift plots from JSON data."""
    raw = load_json("../simulations/avg-drift.json")

    # Process data into a list of dictionaries
    data = []
    for item in raw:
        distribution = item['distribution']
        n = int(item['sampleSize'])
        for estimator, drift_value in item['drifts'].items():
            data.append({
                'distribution': distribution,
                'n': n,
                'estimator': estimator.lower(),
                'drift': float(drift_value)
            })

    # Get unique distributions
    distributions = sorted(set(item['distribution'] for item in data))

    # Generate plots for each distribution
    for distribution in distributions:
        def make_plot():
            fig, ax = plt.subplots(figsize=(8, 4.8))

            # Filter data for this distribution
            dist_data = [d for d in data if d['distribution'] == distribution]

            # Group by estimator
            estimators = {'center': [], 'mean': [], 'median': []}
            for d in dist_data:
                est = d['estimator'].lower()
                if est in estimators:
                    estimators[est].append(d)

            # Plot each estimator
            colors = {'center': CBP['green'], 'mean': CBP['blue'], 'median': CBP['red']}
            labels = {'center': 'Center', 'mean': 'Mean', 'median': 'Median'}

            for est_name in ['center', 'mean', 'median']:
                if estimators[est_name]:
                    est_data = sorted(estimators[est_name], key=lambda x: x['n'])
                    n_values = [d['n'] for d in est_data]
                    drift2_values = [d['drift']**2 for d in est_data]
                    ax.scatter(n_values, drift2_values, color=colors[est_name],
                              label=labels[est_name], s=50, alpha=0.8, zorder=3)

            # Set labels and title
            ax.set_xlabel('n')
            ax.set_ylabel('Drift²')
            ax.set_title(f'{distribution} distribution')

            # Set y-axis limits starting from 0
            ax.set_ylim(bottom=0)

            # Add legend
            ax.legend(title='Estimator', loc='best')

            # Grid
            ax.grid(True, alpha=0.3, zorder=0)

            return fig

        # Save the plot
        name = f"avg-drift-{distribution.lower().strip()}"
        save_plot(name, plot_func=make_plot)


def generate_disp_drift():
    """Generate dispersion drift plots from JSON data."""
    raw = load_json("../simulations/disp-drift.json")

    # Process data into a list of dictionaries
    data = []
    for item in raw:
        distribution = item['distribution']
        n = int(item['sampleSize'])
        for estimator, drift_value in item['drifts'].items():
            data.append({
                'distribution': distribution,
                'n': n,
                'estimator': estimator.lower(),
                'drift': float(drift_value)
            })

    # Get unique distributions
    distributions = sorted(set(item['distribution'] for item in data))

    # Generate plots for each distribution
    for distribution in distributions:
        def make_plot():
            fig, ax = plt.subplots(figsize=(8, 4.8))

            # Filter data for this distribution
            dist_data = [d for d in data if d['distribution'] == distribution]

            # Group by estimator
            estimators = {'spread': [], 'stddev': [], 'mad': []}
            for d in dist_data:
                est = d['estimator'].lower()
                if est in estimators:
                    estimators[est].append(d)

            # Plot each estimator
            colors = {'spread': CBP['green'], 'stddev': CBP['blue'], 'mad': CBP['red']}
            labels = {'spread': 'Spread', 'stddev': 'StdDev', 'mad': 'MAD'}

            for est_name in ['spread', 'stddev', 'mad']:
                if estimators[est_name]:
                    est_data = sorted(estimators[est_name], key=lambda x: x['n'])
                    n_values = [d['n'] for d in est_data]
                    drift2_values = [d['drift']**2 for d in est_data]
                    ax.scatter(n_values, drift2_values, color=colors[est_name],
                              label=labels[est_name], s=50, alpha=0.8, zorder=3)

            # Set labels and title
            ax.set_xlabel('n')
            ax.set_ylabel('Drift²')
            ax.set_title(f'{distribution} distribution')

            # Set y-axis limits starting from 0
            ax.set_ylim(bottom=0)

            # Add legend
            ax.legend(title='Estimator', loc='best')

            # Grid
            ax.grid(True, alpha=0.3, zorder=0)

            return fig

        # Save the plot
        name = f"disp-drift-{distribution.lower().strip()}"
        save_plot(name, plot_func=make_plot)


def figure_distribution_additive():
    """Generate density plot for Additive(0, 1) distribution (normal)."""
    fig, ax = plt.subplots(figsize=(8, 4.8))

    x = np.linspace(-3, 3, 600)
    y = stats.norm.pdf(x, loc=0, scale=1)

    # Use current text color from style
    line_color = plt.rcParams['text.color']
    ax.plot(x, y, color=line_color, linewidth=2)
    ax.set_xlabel('x')
    ax.set_ylabel('Density')
    ax.set_title('Density of Additive(0, 1)')
    ax.grid(True, alpha=0.3, zorder=0)

    return fig


def figure_distribution_multiplic():
    """Generate density plot for Multiplic(0, 1) distribution (lognormal)."""
    fig, ax = plt.subplots(figsize=(8, 4.8))

    x = np.linspace(0.01, 5, 600)
    y = stats.lognorm.pdf(x, s=1, scale=1)

    line_color = plt.rcParams['text.color']
    ax.plot(x, y, color=line_color, linewidth=2)
    ax.set_xlabel('x')
    ax.set_ylabel('Density')
    ax.set_title('Density of Multiplic(0, 1)')
    ax.grid(True, alpha=0.3, zorder=0)

    return fig


def figure_distribution_exponential():
    """Generate density plot for Exponential(1) distribution."""
    fig, ax = plt.subplots(figsize=(8, 4.8))

    x = np.linspace(0, 5, 600)
    y = stats.expon.pdf(x, scale=1)

    line_color = plt.rcParams['text.color']
    ax.plot(x, y, color=line_color, linewidth=2)
    ax.set_xlabel('x')
    ax.set_ylabel('Density')
    ax.set_title('Density of Exponential(1)')
    ax.grid(True, alpha=0.3, zorder=0)

    return fig


def figure_distribution_power():
    """Generate density plot for Power(1, 2) distribution (Pareto)."""
    fig, ax = plt.subplots(figsize=(8, 4.8))

    x = np.linspace(1, 10, 600)
    # Pareto density with scale=1, shape=2: f(x) = 2 * 1^2 / x^3
    y = 2 * (1**2) / (x**3)

    line_color = plt.rcParams['text.color']
    ax.plot(x, y, color=line_color, linewidth=2)
    ax.set_xlabel('x')
    ax.set_ylabel('Density')
    ax.set_title('Density of Power(1, 2)')
    ax.set_ylim(bottom=0)
    ax.grid(True, alpha=0.3, zorder=0)

    return fig


def figure_distribution_uniform():
    """Generate density plot for Uniform(0, 1) distribution."""
    fig, ax = plt.subplots(figsize=(8, 4.8))

    x = np.linspace(-0.5, 1.5, 600)
    y = stats.uniform.pdf(x, loc=0, scale=1)

    line_color = plt.rcParams['text.color']
    ax.plot(x, y, color=line_color, linewidth=2)
    ax.set_xlabel('x')
    ax.set_ylabel('Density')
    ax.set_title('Density of Uniform(0, 1)')
    ax.grid(True, alpha=0.3, zorder=0)

    return fig


def regenerate_figures():
    """Remove existing images and regenerate all distribution figures."""
    # Remove all existing images (except logo.png)
    for pattern in ['*.png', '*.jpg', '*.svg']:
        for file in glob.glob(pattern):
            if file != 'logo.png':
                Path(file).unlink()

    # Generate all figures
    figures = {
        'distribution-additive': figure_distribution_additive,
        'distribution-multiplic': figure_distribution_multiplic,
        'distribution-exponential': figure_distribution_exponential,
        'distribution-power': figure_distribution_power,
        'distribution-uniform': figure_distribution_uniform,
    }

    for name, func in figures.items():
        save_plot(name, plot_func=func)


if __name__ == '__main__':
    regenerate_figures()
    generate_avg_drift()
    generate_disp_drift()
    print("\nAll images generated successfully!")
