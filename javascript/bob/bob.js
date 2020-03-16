export const hey = message => {
  message = message.trim();

  const silent = message === "";
  const yelling = /[A-Z]/.test(message) && !/[a-z]/.test(message);
  const question = message.endsWith('?');
  
  if (silent) {
    return "Fine. Be that way!";
  } else if (yelling && question) {
    return "Calm down, I know what I'm doing!";
  } else if (question) {
    return "Sure.";
  } else if (yelling) {
    return "Whoa, chill out!";
  } else {
    return "Whatever."
  }
};
