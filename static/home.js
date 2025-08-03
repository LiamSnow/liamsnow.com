let container = document.querySelector("#links");

let links = container.children;
let selectedIndex = 0;
links[selectedIndex].classList.add("selected");
const downKeys = ["j", "ArrowDown"];
const upKeys = ["k", "ArrowUp"];

const shortcuts = Array.from(links).map(link => link.children[1].textContent);

document.addEventListener("keydown", function(event) {
  if (event.key === "Enter") {
      links[selectedIndex].click();
      return
  }
  if (downKeys.includes(event.key)) {
      moveCursor(true);
      return
  }
  if (upKeys.includes(event.key)) {
      moveCursor(false);
      return
  }
  if (shortcuts.includes(event.key)) {
    const index = shortcuts.indexOf(event.key);
    if (index !== -1) {
      links[index].click();
    }
  }
});

function moveCursor(down) {
  links[selectedIndex].classList.remove("selected");
  if (down) {
    selectedIndex = (selectedIndex + 1) % links.length;
  }
  else {
    selectedIndex = (selectedIndex - 1 + links.length) % links.length;
  }
  links[selectedIndex].classList.add("selected");
}
