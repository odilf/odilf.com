let buttons = {
  book: document.getElementById("books-tab"),
  movie: document.getElementById("movies-tab"),
  videogame: document.getElementById("videogames-tab"),
  music: document.getElementById("music-tab"),
  all: document.getElementById("all-tab"),
};

let active_tab = buttons.all;

for (const name in buttons) {
  buttons[name].addEventListener("click", () => {
    active_tab.disabled = false;
    active_tab = buttons[name];
    active_tab.disabled = true;

    if (name === "all") {
      document.querySelectorAll(".media-log-entry").forEach((item) => {
        item.style = "";
      });
    } else {
      document.querySelectorAll(".media-log-entry").forEach((item) => {
        item.style = "display: none;";
      });

      document.querySelectorAll(`.topic-${name}`).forEach((item) => {
        item.style = "";
      });
    }
  });
}
