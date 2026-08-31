(function () {
    "use strict";

    const root = document.documentElement;
    const themeButton = document.getElementById("theme-toggle");
    const themeStorageKey = "microbin_theme";
    const systemTheme = window.matchMedia("(prefers-color-scheme: dark)");

    function storedTheme() {
        try {
            return localStorage.getItem(themeStorageKey);
        } catch (_) {
            return null;
        }
    }

    function applyTheme(theme) {
        root.dataset.theme = theme || (systemTheme.matches ? "dark" : "light");
        if (themeButton) {
            const dark = root.dataset.theme === "dark";
            themeButton.textContent = dark ? "☀" : "☾";
            themeButton.setAttribute("aria-label", dark ? themeButton.dataset.lightLabel : themeButton.dataset.darkLabel);
            themeButton.setAttribute("title", dark ? themeButton.dataset.lightLabel : themeButton.dataset.darkLabel);
        }
    }

    applyTheme(storedTheme());
    systemTheme.addEventListener("change", function () {
        if (!storedTheme()) applyTheme(null);
    });
    if (themeButton) {
        themeButton.addEventListener("click", function () {
            const next = root.dataset.theme === "dark" ? "light" : "dark";
            try { localStorage.setItem(themeStorageKey, next); } catch (_) { /* best effort */ }
            applyTheme(next);
        });
    }

    const keyInput = document.getElementById("custom-key");
    const keyStatus = document.getElementById("custom-key-status");
    if (keyInput && keyStatus) {
        const updateKeyStatus = function () {
            const value = keyInput.value.trim();
            const valid = value === "" || /^[a-z0-9_-]{3,64}$/.test(value);
            keyStatus.textContent = value === "" ? keyStatus.dataset.emptyLabel : (valid ? keyStatus.dataset.validLabel : keyStatus.dataset.invalidLabel);
            keyStatus.classList.toggle("is-valid", valid && value !== "");
            keyStatus.classList.toggle("is-invalid", !valid);
        };
        keyInput.addEventListener("input", updateKeyStatus);
        updateKeyStatus();
    }

    const contentInput = document.querySelector(".content-field textarea");
    const contentCounter = document.getElementById("content-counter");
    if (contentInput && contentCounter) {
        const updateCounter = function () { contentCounter.textContent = contentInput.value.length + " " + contentCounter.dataset.suffix; };
        contentInput.addEventListener("input", updateCounter);
        updateCounter();
    }

    const dropZone = document.getElementById("drop-zone");
    const fileInput = document.getElementById("file");
    const fileName = document.getElementById("file-name");
    const clearFile = document.getElementById("clear-file");
    if (dropZone && fileInput) {
        const showFile = function () {
            const file = fileInput.files && fileInput.files[0];
            if (fileName) fileName.textContent = file ? file.name : fileName.dataset.emptyLabel;
            if (clearFile) clearFile.hidden = !file;
        };
        ["dragenter", "dragover"].forEach(function (event) {
            dropZone.addEventListener(event, function (e) { e.preventDefault(); dropZone.classList.add("is-dragover"); });
        });
        ["dragleave", "drop"].forEach(function (event) {
            dropZone.addEventListener(event, function (e) { e.preventDefault(); dropZone.classList.remove("is-dragover"); });
        });
        dropZone.addEventListener("drop", function (e) {
            if (e.dataTransfer.files.length) fileInput.files = e.dataTransfer.files;
            showFile();
        });
        fileInput.addEventListener("change", showFile);
        if (clearFile) clearFile.addEventListener("click", function () { fileInput.value = ""; showFile(); });
        showFile();
    }

    const form = document.querySelector("#pasta-form, .submit-form");
    const submit = form && form.querySelector("button[type=submit], input[type=submit]");
    if (form && submit) {
        form.addEventListener("submit", function () {
            if (!form.checkValidity()) return;
            submit.disabled = true;
            submit.dataset.originalLabel = submit.value || submit.textContent;
            if ("value" in submit) submit.value = submit.dataset.savingLabel;
            else submit.textContent = submit.dataset.savingLabel;
        });
    }

    document.querySelectorAll("[data-confirm-remove]").forEach(function (link) {
        link.addEventListener("click", function (event) {
            if (!window.confirm(link.dataset.confirmRemove)) event.preventDefault();
        });
    });
})();
