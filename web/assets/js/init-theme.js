const themeToggleSystemIcon = document.getElementById('theme-toggle-system-icon');
const themeToggleDarkIcon = document.getElementById('theme-toggle-dark-icon');
const themeToggleLightIcon = document.getElementById('theme-toggle-light-icon');
const imagesDark = document.querySelectorAll('.img-dark');
const imagesLight = document.querySelectorAll('.img-light');

function getTheme() {
  if (localStorage.getItem('color-theme')) {
    return localStorage.getItem('color-theme');
  }
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

function applyTheme(theme) {
  const currentTheme = theme || getTheme();
  const isDark = currentTheme === 'dark';
  const isSystem = localStorage.getItem('color-theme') === null;
  
  document.documentElement.classList.toggle('dark', isDark);
  document.documentElement.setAttribute('data-theme', isDark ? 'dark' : 'light');
  
  // Show the appropriate icon based on current state
  themeToggleSystemIcon.classList.toggle('hidden', !isSystem);
  themeToggleDarkIcon.classList.toggle('hidden', isSystem || isDark);
  themeToggleLightIcon.classList.toggle('hidden', isSystem || !isDark);
  
  imagesDark.forEach(image => image.classList.toggle('hidden', !isDark));
  imagesLight.forEach(image => image.classList.toggle('hidden', isDark));
}

// Init
applyTheme();

document.getElementById('theme-toggle').addEventListener('click', (event) => {
  const currentTheme = localStorage.getItem('color-theme');
  const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  
  if (currentTheme === null) {
    // System → First non-system theme (Light if system is dark, Dark if system is light)
    localStorage.setItem('color-theme', systemPrefersDark ? 'light' : 'dark');
  } else if (currentTheme === (systemPrefersDark ? 'light' : 'dark')) {
    // First non-system theme → Second non-system theme
    localStorage.setItem('color-theme', systemPrefersDark ? 'dark' : 'light');
  } else {
    // Second non-system theme → System
    localStorage.removeItem('color-theme');
  }
  
  applyTheme();
});

// Listen for OS theme changes
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
  if (!localStorage.getItem('color-theme')) {
    applyTheme(e.matches ? 'dark' : 'light');
  }
});
