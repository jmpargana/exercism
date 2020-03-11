export const isPangram = (sentence) => {
  const letters = Array.from(sentence.toLowerCase()).filter(c => /[a-z]/.test(c));
  const set = new Set(letters)
  return set.size == 26
};
