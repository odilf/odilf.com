let buttons = {
  dev: document.getElementById("development-tab"),
  personal: document.getElementById("personal-tab"),
  all: document.getElementById("all-tab"),
};

let active_tab = buttons.all;

for (const name in buttons) {
  buttons[name].addEventListener("click", () => {
    console.log("Clicked", name);
    active_tab.disabled = false;
    active_tab = buttons[name];
    active_tab.disabled = true;

    if (name === "all") {
      document.querySelectorAll(".blog-entry").forEach((item) => {
        item.style = "";
      });
    } else {
      document.querySelectorAll(".blog-entry").forEach((item) => {
        item.style = "display: none;";
      });

      document.querySelectorAll(`.topic-${name}`).forEach((item) => {
        item.style = "";
      });
    }
  });
}
