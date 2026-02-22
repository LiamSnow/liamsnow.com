window.addEventListener('load', () => {
  const HOUR = 60 * 60;
  const DAY = HOUR * 24;
  
  const UNITS = [
    // { label: "year", seconds: DAY * 365 },
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
});
