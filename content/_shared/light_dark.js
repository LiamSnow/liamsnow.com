window.addEventListener('load', () => {
  var button = document.querySelector('.light-dark');
  if (!button) {
    console.error("Could not find light-dark button!");
    return
  };

  var moonImg = button.querySelector('.moon');
  var sunImg = button.querySelector('.sun');

  function getTheme() {
    var inline = document.documentElement.style.colorScheme;
    if (inline === 'light' || inline === 'dark') return inline;
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }

  function updateImages(theme) {
    if (theme === 'dark') {
      moonImg.style.display = 'none';
      sunImg.style.display = '';
    } else {
      moonImg.style.display = '';
      sunImg.style.display = 'none';
    }
  }

  function setTheme(theme) {
    document.documentElement.style.colorScheme = theme;
    document.documentElement.setAttribute('data-theme', theme);
    localStorage.setItem('theme', theme);
    updateImages(theme);
  }

  var saved = localStorage.getItem('theme');
  if (saved) {
    setTheme(saved);
  } else {
    updateImages(getTheme());
  }

  button.addEventListener('click', function() {
    var next = getTheme() === 'dark' ? 'light' : 'dark';

    if (document.startViewTransition) {
      document.startViewTransition(function() {
        setTheme(next);
      });
    } else {
      setTheme(next);
    }
  });
});
