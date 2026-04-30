window.addEventListener('load', () => {
  const links = document.querySelectorAll('a[href]');
  const prefetched = new Set();
  let hoverTimer = null;

  links.forEach(link => {
    const href = link.getAttribute('href');

    if (href && isLocalUrl(href)) {
      link.addEventListener('mouseenter', handleMouseEnter);
      link.addEventListener('mouseleave', handleMouseLeave);
    }
  });

  function isLocalUrl(url) {
    if (url.startsWith('/')) {
      return true;
    }

    let urlObj;
    try {
      urlObj = new URL(url, window.location.origin);
    } catch (error) {
      return false;
    }

    const isLocal = urlObj.origin === window.location.origin;
    return isLocal;
  }

  function prefetchUrl(url) {
    if (prefetched.has(url)) {
      return;
    }

    if (url === window.location.pathname) {
      return;
    }

    const link = document.createElement('link');
    link.rel = 'prefetch';
    link.href = url;
    document.head.appendChild(link);

    prefetched.add(url);
  }

  function handleMouseEnter(event) {
    const link = event.currentTarget;
    const href = link.getAttribute('href');

    if (!href || !isLocalUrl(href)) {
      return;
    }

    if (hoverTimer) {
      clearTimeout(hoverTimer);
    }

    hoverTimer = setTimeout(() => {
      prefetchUrl(href);
    }, 100);
  }

  function handleMouseLeave() {
    if (hoverTimer) {
      clearTimeout(hoverTimer);
      hoverTimer = null;
    }
  }
});
