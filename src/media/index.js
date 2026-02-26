const typeButtons = {
  book: document.getElementById("books-tab"),
  movie: document.getElementById("movies-tab"),
  videogame: document.getElementById("videogames-tab"),
  music: document.getElementById("music-tab"),
  all: document.getElementById("all-tab"),
};
let activeTypeTab = typeButtons.all;
let selectedType = "all";

const starButtons = document.querySelectorAll(".star-btn");
let selectedStarRating = 0;

function applyFilters() {
  document.querySelectorAll(".media-log-entry").forEach((item) => {
    const typeFilter = () =>
      selectedType === "all" || item.dataset.mediaType === selectedType;
    const starFilter = () =>
      parseFloat(item.dataset.rating) >= selectedStarRating;

    const show = typeFilter() && starFilter();

    item.style.display = show ? "" : "none";
  });
}

for (const name in typeButtons) {
  typeButtons[name].addEventListener("click", () => {
    activeTypeTab.disabled = false;
    activeTypeTab = typeButtons[name];
    activeTypeTab.disabled = true;
    selectedType = name;
    applyFilters();
  });
}

for (const star of starButtons) {
  const rating = parseInt(star.dataset.rating);
  star.addEventListener("click", () => {
    if (selectedStarRating === rating) {
      selectedStarRating = 0;
    } else {
      selectedStarRating = rating;
    }

    // Update styles
    for (let i = 0; i < selectedStarRating; i += 1) {
      starButtons[i].dataset.active = true;
    }
    for (let i = selectedStarRating; i < starButtons.length; i += 1) {
      delete starButtons[i].dataset.active;
      starButtons[i].removeAttribute("data-active");
    }

    applyFilters();
  });
}
