document.onscroll = () => {
  const header = document.querySelector("header");
  if (window.scrollY > 0) {
    header.classList.add("scroll");
  } else {
    header.classList.remove("scroll");
  }
};
