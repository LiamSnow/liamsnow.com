document.addEventListener("DOMContentLoaded", function() {
  document.querySelectorAll('span[data-math-style]').forEach(function(elem) {
    const mathStyle = elem.getAttribute('data-math-style');
    const tex = elem.textContent;
    
    try {
      katex.render(tex, elem, {
        displayMode: mathStyle === 'display',
        throwOnError: false
      });
    } catch (e) {
      console.error('KaTeX error:', e);
    }
  });
});
