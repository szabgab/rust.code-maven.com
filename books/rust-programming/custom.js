function linkIncludeFiles() {
  document.querySelectorAll("div[data-embedify][data-app='include']").forEach((node) => {
    const file = node.getAttribute("data-option-file");
    if (!file) return;

    // Only handle files inside src/examples/
    if (!file.startsWith("src/examples/")) return;

    // Only handle .rs and .py files
    if (!file.endsWith(".rs") && !file.endsWith(".py")) return;

    // Create a link to the corresponding GitHub file
    const a = document.createElement("a");
    a.href = `https://github.com/szabgab/rust.code-maven.com/tree/main/books/rust-programming/${file}`;
    a.textContent = file;
    a.target = "_blank";

    // Insert the link after the current node
    node.insertAdjacentElement("afterend", a);
  });
}

document.addEventListener("DOMContentLoaded", linkIncludeFiles);
