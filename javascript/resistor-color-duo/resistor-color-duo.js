export const decodedValue = ([firstColor, secondColor]) => {
  return parseInt(
    resistorColors
      .indexOf(firstColor)
      .toString(10)
      .concat(resistorColors.indexOf(secondColor).toString(10))
  );
};

export const resistorColors = [
  "black",
  "brown",
  "red",
  "orange",
  "yellow",
  "green",
  "blue",
  "violet",
  "grey",
  "white"
];
