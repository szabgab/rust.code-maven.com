const extensions = [".rs", ".py", ".toml"];
const base_url = "https://github.com/szabgab/rust.code-maven.com/tree/main/books/rust-programming/"

function linkIncludeFiles() {
  document.querySelectorAll("div[data-embedify][data-app='include']").forEach((node) => {
    const file = node.getAttribute("data-option-file");
    console.log(`Try ${file}`);
    if (!file) return;

    // Only handle files inside src/examples/
    if (!file.startsWith("src/examples/")) return;

    extensions.forEach((ext) => {
        if (file.endsWith(ext)) {
            createLink(node, file);
            return;
        }
    });
  });
}

function createLink(node, file) {
    // Create a link to the corresponding GitHub file
    const a = document.createElement("a");
    a.href = `${base_url}${file}`;
    a.textContent = file;
    a.target = "_blank";

    console.log("Insert the link after the current node");
    node.insertAdjacentElement("afterend", a);
}

document.addEventListener("DOMContentLoaded", linkIncludeFiles);
