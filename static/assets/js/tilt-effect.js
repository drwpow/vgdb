const TILT_X = 18;
const TILT_Y = 17;
const TINT_Y = 0.4;

function onMousemove(evt) {
  if (!evt.target) return;

  const wrapper = evt.target.closest(".gameBox\\/boxArt");
  const boxArt = wrapper.querySelector(".gameBox\\/boxArt\\/img");

  const { clientX, clientY } = evt;
  const { x, y, width, height } = wrapper.getBoundingClientRect();
  const centerX = width / 2;
  const centerY = height / 2;
  const deltaX = (clientX - x - centerX) / width;
  const deltaY = (clientY - y - centerY) / height;
  wrapper.style.setProperty("--tilt-x", `${TILT_X * deltaX}deg`);
  wrapper.style.setProperty("--tilt-y", `${-TILT_Y * deltaY}deg`);
  boxArt.style.setProperty("--brightness-y", `${100 - TINT_Y * deltaY * 100}%`);
}

function onMouseleave(evt) {
  if (!evt.target) return;

  const wrapper = evt.target.closest(".gameBox\\/boxArt");
  const boxArt = wrapper.querySelector(".gameBox\\/boxArt\\/img");

  wrapper.style.setProperty("--tilt-x", 0);
  wrapper.style.setProperty("--tilt-y", 0);
  boxArt.style.setProperty("--brightness-y", "100%");
}

export function init() {
  document.querySelectorAll("[data-tilt]").forEach((el) => {
    el.addEventListener("mousemove", onMousemove);
    el.addEventListener("mouseleave", onMouseleave);
  });
}

init();
