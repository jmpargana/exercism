export const gigasecond = (startDate) => {
  return new Date(startDate.getTime() + Math.pow(10, 9))
};
