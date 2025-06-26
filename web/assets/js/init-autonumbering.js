document.addEventListener('DOMContentLoaded', function() {
  function processEntities(entity) {
    const entitySpans = document.querySelectorAll(`span.autoanchor_${entity}`);
    entitySpans.forEach((span, index) => {
      span.textContent = index + 1;
    });
    const entityLinks = document.querySelectorAll(`a[href^="#${entity}_"]`);
    entityLinks.forEach(link => {
      const refId = link.getAttribute('href').substring(1);
      const matchingSpan = document.querySelector(`span.autoanchor_${entity}[data-ref="${refId}"]`);
      if (matchingSpan) {
        link.textContent = matchingSpan.textContent;
      }
    });
  }

  // Process all entities
  ['fig', 'exm', 'tbl'].forEach(processEntities);
});
