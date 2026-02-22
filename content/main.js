// header shadow
document.onscroll = () => {
  const header = document.querySelector("header");
  if (window.scrollY > 0) {
    header.classList.add("scroll");
  } else {
    header.classList.remove("scroll");
  }
};

// convert all fixed time (.date) -> relative time
// TODO eventually should bake into SSG
const HOUR = 60 * 60;
const DAY = HOUR * 24;

const UNITS = [
  { label: "month", seconds: DAY * 30 },
  { label: "week", seconds: DAY * 7 },
  { label: "day", seconds: DAY },
];

function getRelativeTime(dateString) {
  const date = new Date(dateString + "T00:00:00");
  const now = new Date();
  const diffSecs = Math.floor((now - date) / 1000);

  if (diffSecs < 0) {
    return "in the future";
  }

  if (diffSecs < 10) {
    return "just now";
  }

  for (const unit of UNITS) {
    const count = Math.floor(diffSecs / unit.seconds);
    if (count >= 1) {
      return `${count} ${unit.label}${count > 1 ? "s" : ""} ago`;
    }
  }

  return "just now";
}

document.querySelectorAll(".date").forEach((el) => {
  const raw = el.textContent.trim();
  if (!/^\d{4}-\d{2}-\d{2}$/.test(raw)) return;

  const fullDate = new Date(raw + "T00:00:00").toLocaleDateString(undefined, {
    year: "numeric",
    month: "long",
    day: "numeric",
  });

  el.textContent = getRelativeTime(raw);
  el.title = fullDate;
});

// light dark
var button = document.querySelector('.light-dark');
if (!button) {
  console.error("Could not find light-dark button!");
} else {
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
}

// McMaster-like preload
const prefetched = new Set();
let hoverTimer = null;

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

  return urlObj.origin === window.location.origin;
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

document.querySelectorAll('a[href]').forEach(link => {
  const href = link.getAttribute('href');

  if (href && isLocalUrl(href)) {
    link.addEventListener('mouseenter', handleMouseEnter);
    link.addEventListener('mouseleave', handleMouseLeave);
  }
});
