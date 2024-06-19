document.addEventListener("htmx:afterSwap", updateDateTimeElements);
document.addEventListener("DOMContentLoaded", updateDateTimeElements);

function updateDateTimeElements() {
  document.querySelectorAll("span[data-format]").forEach((span) => {
    span.removeAttribute('data-format')
    span.innerText = new Date(span.innerText).toLocaleString()
  });
}
