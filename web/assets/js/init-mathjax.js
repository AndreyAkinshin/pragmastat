window.MathJax = {
  loader: {load: ['[tex]/tagformat', '[tex]/textmacros']},
  tex: {
    tags: 'ams',
    tagformat: {
      number: (n) => n.toString(),
      tag:    (tag) => '(' + tag + ')',
      id:     (id) => id.replace(/\s/g, '_'),
      url:    (id, base) => base + '#' + encodeURIComponent(id)
    },
    packages: {'[+]': ['tagformat', 'textmacros']},
    inlineMath: [['$', '$'], ['\\(', '\\)']],
    displayMath: [['$$', '$$'], ['\\[', '\\]']],
    processEscapes: true,
    processEnvironments: true
  },
  options: {
    skipHtmlTags: ['script', 'noscript', 'style', 'textarea', 'pre'],
    ignoreHtmlClass: 'tex2jax_ignore',
    processHtmlClass: 'tex2jax_process'
  },
  svg: {
    fontCache: 'global'
  }
};
window.addEventListener('load', (event) => {
  document.querySelectorAll("mjx-container").forEach(function(x){
    x.parentElement.classList += ' has-jax'})
});