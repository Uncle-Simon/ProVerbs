const { invoke } = window.__TAURI__.tauri;

let verseInputEl;
let bibleCorrectEl;
let biblePercentEl;
let memoryVerseEl;

async function submitAnswer() {
  memoryVerseEl.style.display = "flex";
  document.querySelector("#verse-form").style.display = "none";
  document.querySelector("#hide").style.display = "";
  document.querySelector("#instruction").textContent = "Memorize the verse below";
  let answers = await invoke("get_answer", {
    answer: verseInputEl.value,
    memoryVerse: memoryVerseEl.textContent,
  });
  if (answers[2]) {
    newQuestion();
  }
  biblePercentEl.textContent = answers[0];
  bibleCorrectEl.textContent = answers[1];
}

async function newQuestion() {
  let questions = await invoke("get_new_question");
  memoryVerseEl.textContent = questions;
}

async function hideVerse() {
  memoryVerseEl.style.display = "none";
  document.querySelector("#verse-form").style.display = "flex";
  document.querySelector("#hide").style.display = "none";
  verseInputEl.focus();
  verseInputEl.select();
  verseInputEl.value = "";
  document.querySelector("#instruction").textContent = "Type the memorized verse";
}

window.addEventListener("DOMContentLoaded", () => {
  verseInputEl = document.querySelector("#verse-input");
  bibleCorrectEl = document.querySelector("#bible-correct");
  biblePercentEl = document.querySelector("#bible-percent");
  memoryVerseEl = document.querySelector("#memory-verse");
  // newQuestion();
  invoke("setup_verses");
  document.querySelector("#verse-form").addEventListener("submit", (e) => {
    e.preventDefault();
    submitAnswer();
  });
  document.querySelector("#hide").addEventListener("click", (e) => {
    hideVerse();
  })
  document.querySelector("#skip").addEventListener("click", (e) => {
    newQuestion();
  })
});
